use rusqlite::{params, Connection, Result};

pub struct Database {
    pub conn: Connection,
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS production_plan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            barcode TEXT UNIQUE NOT NULL,
            part_no TEXT NOT NULL,
            side TEXT NOT NULL,
            plan_qty INTEGER NOT NULL,
            current_running INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        ",
        [],
    )?;

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS print_log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            plan_id INTEGER NOT NULL,
            barcode TEXT NOT NULL,
            running_no INTEGER NOT NULL,
            printed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            UNIQUE (plan_id, running_no)
        )
        ",
        [],
    )?;

    Ok(())
}

impl Database {
    pub fn get_or_create_plan(
        &self,
        barcode: &str,
        part_no: &str,
        side: &str,
        plan_qty: i32,
    ) -> Result<()> {
        self.conn.execute(
            "
            INSERT OR IGNORE INTO production_plan
            (barcode, part_no, side, plan_qty)
            VALUES (?1, ?2, ?3, ?4)
            ",
            params![barcode, part_no, side, plan_qty],
        )?;
        Ok(())
    }

    pub fn consume_next_running(&mut self, barcode: &str) -> Result<i32> {
        let tx = self.conn.transaction()?;

        let (plan_id, current, plan_qty): (i64, i32, i32) =
            tx.query_row(
                "
                SELECT id, current_running, plan_qty
                FROM production_plan
                WHERE barcode = ?1
                ",
                params![barcode],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )?;

        if current >= plan_qty {
            return Err(rusqlite::Error::ExecuteReturnedResults);
        }

        let next = current + 1;

        tx.execute(
            "
            INSERT INTO print_log (plan_id, barcode, running_no)
            VALUES (?1, ?2, ?3)
            ",
            params![plan_id, barcode, next],
        )?;

        tx.execute(
            "
            UPDATE production_plan
            SET current_running = ?1
            WHERE id = ?2
            ",
            params![next, plan_id],
        )?;

        tx.commit()?;
        Ok(next)
    }
}