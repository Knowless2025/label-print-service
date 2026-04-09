use rusqlite::{params, Connection, Result};

pub struct Database {
    pub conn: Connection,
}

/// สร้างตารางทั้งหมด
pub fn init_db(conn: &Connection) -> Result<()> {
    
    // ========================================
    // ตาราง 1: Production Plans
    // ========================================
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS production_plans (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            prod_id TEXT UNIQUE NOT NULL,
            item_id TEXT NOT NULL,
            brand_name TEXT NOT NULL,
            side TEXT NOT NULL,
            plan_qty INTEGER NOT NULL,
            total_produced INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'ACTIVE',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            completed_at DATETIME
        )
        ",
        [],
    )?;

    // ========================================
    // ตาราง 2: Batch Records (แต่ละกะ)
    // ========================================
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS batch_records (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            plan_id INTEGER NOT NULL,
            batch_no TEXT UNIQUE NOT NULL,
            machine_id TEXT NOT NULL,
            shift INTEGER NOT NULL,
            date TEXT NOT NULL,
            produced_qty INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'ACTIVE',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            completed_at DATETIME,
            FOREIGN KEY (plan_id) REFERENCES production_plans(id)
        )
        ",
        [],
    )?;

    // ========================================
    // ตาราง 3: Print Logs (ทุกการพิมพ์)
    // ========================================
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS print_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            batch_id INTEGER NOT NULL,
            plan_id INTEGER NOT NULL,
            running_no INTEGER NOT NULL,
            global_running INTEGER NOT NULL,
            qr_code TEXT UNIQUE NOT NULL,
            prod_id TEXT NOT NULL,
            item_id TEXT NOT NULL,
            brand_name TEXT NOT NULL,
            side TEXT NOT NULL,
            batch_no TEXT NOT NULL,
            machine_id TEXT NOT NULL,
            shift INTEGER NOT NULL,
            printed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (batch_id) REFERENCES batch_records(id),
            FOREIGN KEY (plan_id) REFERENCES production_plans(id)
        )
        ",
        [],
    )?;

    // ========================================
    // สร้าง Indexes
    // ========================================
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_plan_status 
         ON production_plans(status)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_batch_plan 
         ON batch_records(plan_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_batch_status 
         ON batch_records(status)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_print_batch 
         ON print_logs(batch_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_print_qr 
         ON print_logs(qr_code)",
        [],
    )?;

    Ok(())
}

/// สร้าง Batch Number จาก วัน + เครื่อง + กะ
/// Format: YYMMDDMM#
/// Example: 260202G71 = 26/02/02, Machine G7, Shift 1
pub fn generate_batch_number(
    date: &str,      // "2026-02-02"
    machine_id: &str, // "G7"
    shift: i32       // 1, 2, 3
) -> String {
    let parts: Vec<&str> = date.split('-').collect();
    let year = &parts[0][2..4];   // "26"
    let month = parts[1];          // "02"
    let day = parts[2];            // "02"
    
    format!("{}{}{}{}{}", year, month, day, machine_id, shift)
}

/// Format Running Number เป็น 4 หลัก (0001, 0002, ...)
pub fn format_running_number(num: i32) -> String {
    format!("{:04}", num)
}

impl Database {
    /// สร้างหรือดึง Production Plan
    pub fn get_or_create_plan(
        &self,
        prod_id: &str,
        item_id: &str,
        brand_name: &str,
        side: &str,
        plan_qty: i32,
    ) -> Result<i64> {
        // ลองดึงก่อน
        match self.conn.query_row(
            "SELECT id FROM production_plans WHERE prod_id = ?1",
            params![prod_id],
            |row| row.get(0),
        ) {
            Ok(id) => Ok(id),
            Err(_) => {
                // ถ้าไม่มี สร้างใหม่
                self.conn.execute(
                    "
                    INSERT INTO production_plans
                    (prod_id, item_id, brand_name, side, plan_qty)
                    VALUES (?1, ?2, ?3, ?4, ?5)
                    ",
                    params![prod_id, item_id, brand_name, side, plan_qty],
                )?;
                
                Ok(self.conn.last_insert_rowid())
            }
        }
    }

    /// สร้าง Batch ใหม่
    pub fn create_batch(
        &self,
        plan_id: i64,
        batch_no: &str,
        machine_id: &str,
        shift: i32,
        date: &str,
    ) -> Result<i64> {
        self.conn.execute(
            "
            INSERT INTO batch_records
            (plan_id, batch_no, machine_id, shift, date)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ",
            params![plan_id, batch_no, machine_id, shift, date],
        )?;
        
        Ok(self.conn.last_insert_rowid())
    }

    /// ดึง Active Batch
    pub fn get_active_batch(&self, plan_id: i64) -> Result<(i64, String, i32)> {
        self.conn.query_row(
            "
            SELECT id, batch_no, produced_qty
            FROM batch_records
            WHERE plan_id = ?1 AND status = 'ACTIVE'
            ORDER BY created_at DESC
            LIMIT 1
            ",
            params![plan_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
    }

    /// พิมพ์ฉลากถัดไป (ตัดยอด + บันทึก)
    pub fn print_next_label(
        &mut self,
        plan_id: i64,
        batch_id: i64,
        qr_code: &str,
        prod_id: &str,
        item_id: &str,
        brand_name: &str,
        side: &str,
        batch_no: &str,
        machine_id: &str,
        shift: i32,
    ) -> Result<(i32, i32, i32)> {
        let tx = self.conn.transaction()?;

        // ดึงข้อมูลปัจจุบัน
        let (batch_running, plan_total, plan_qty): (i32, i32, i32) = tx.query_row(
            "
            SELECT 
                b.produced_qty,
                p.total_produced,
                p.plan_qty
            FROM batch_records b
            JOIN production_plans p ON p.id = b.plan_id
            WHERE b.id = ?1 AND p.id = ?2
            ",
            params![batch_id, plan_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )?;

        // เช็คว่าเกินแผนไหม
        if plan_total >= plan_qty {
            return Err(rusqlite::Error::ExecuteReturnedResults);
        }

        let running_no = batch_running + 1;
        let global_running = plan_total + 1;

        // บันทึก print log
        tx.execute(
            "
            INSERT INTO print_logs
            (batch_id, plan_id, running_no, global_running, qr_code,
             prod_id, item_id, brand_name, side, batch_no, machine_id, shift)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
            ",
            params![
                batch_id, plan_id, running_no, global_running, qr_code,
                prod_id, item_id, brand_name, side, batch_no, machine_id, shift
            ],
        )?;

        // อัปเดต batch
        tx.execute(
            "UPDATE batch_records SET produced_qty = ?1 WHERE id = ?2",
            params![running_no, batch_id],
        )?;

        // อัปเดต plan
        tx.execute(
            "UPDATE production_plans SET total_produced = ?1 WHERE id = ?2",
            params![global_running, plan_id],
        )?;

        // ถ้าครบแผน ปิด
        if global_running >= plan_qty {
            tx.execute(
                "UPDATE production_plans SET status = 'COMPLETED', 
                 completed_at = CURRENT_TIMESTAMP WHERE id = ?1",
                params![plan_id],
            )?;
            tx.execute(
                "UPDATE batch_records SET status = 'COMPLETED',
                 completed_at = CURRENT_TIMESTAMP WHERE id = ?1",
                params![batch_id],
            )?;
        }

        tx.commit()?;

        let remaining = plan_qty - global_running;
        Ok((running_no, global_running, remaining))
    }

    /// ดึงสถานะการผลิต
    pub fn get_production_status(&self, prod_id: &str) -> Result<(String, String, String, String, i32, i32, String)> {
        self.conn.query_row(
            "
            SELECT 
                p.prod_id,
                p.item_id,
                p.brand_name,
                p.side,
                p.total_produced,
                p.plan_qty,
                p.status
            FROM production_plans p
            WHERE p.prod_id = ?1
            ",
            params![prod_id],
            |row| Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_number_generation() {
        let batch = generate_batch_number("2026-02-02", "G7", 1);
        assert_eq!(batch, "260202G71");
        
        let batch2 = generate_batch_number("2026-02-14", "D9", 2);
        assert_eq!(batch2, "260214D92");
    }

    #[test]
    fn test_running_number_format() {
        assert_eq!(format_running_number(1), "0001");
        assert_eq!(format_running_number(69), "0069");
        assert_eq!(format_running_number(100), "0100");
    }
}