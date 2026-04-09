use crate::models::CompleteLabelData;
use crate::qr;

/// สร้าง ZPL Code สำหรับฉลาก 2x2 นิ้ว (Layout C)
/// 
/// Layout:
/// ┌─────────────────────────────────┐
/// │ Ford                        LH  │
/// │                           0001  │
/// │ PROD ID: P992383                │
/// │ ITEM: 7113800055                │
/// │ BATCH: 260202G71                │
/// │                                 │
/// │         ████████████            │
/// │         ████ QR █████           │
/// │         ████████████            │
/// └─────────────────────────────────┘
///
pub fn generate_zpl(label: &CompleteLabelData) -> String {
    // สร้าง QR Code data
    let qr_data = qr::generate_qr_data(
        &label.item_id,
        &label.batch_no,
        &label.running_no,
        &label.side,
        &label.prod_id,
    );

    format!(
r#"^XA
^CI28
^PW400
^LL400

~TA000
~JSN
^LT0
^MNW
^MTT
^PON
^PMN
^LH0,0
^JMA
^PR4,4
~SD15
^JUS
^LRN
^XZ

^XA
^MMT
^PW400
^LL400
^LS0

^FO20,20^A0N,30,30^FH\^FD{}^FS
^FO350,20^A0N,36,36^FH\^FD{}^FS

^FO350,60^A0N,28,28^FH\^FD{}^FS

^FO20,100^A0N,24,24^FH\^FDPROD ID: {}^FS
^FO20,130^A0N,24,24^FH\^FDITEM: {}^FS
^FO20,160^A0N,24,24^FH\^FDBATCH: {}^FS

^FO100,210^BQN,2,6
^FDQA,{}^FS

^PQ1,0,1,Y
^XZ"#,
        label.brand_name,   // Ford
        label.side,         // LH
        label.running_no,   // 0001
        label.prod_id,      // P992383
        label.item_id,      // 7113800055
        label.batch_no,     // 260202G71
        qr_data             // ARKR-7113800055-260202G71-0001-LH-P992383
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zpl_generation() {
        let label = CompleteLabelData {
            prod_id: "P992383".to_string(),
            item_id: "7113800055".to_string(),
            brand_name: "Ford".to_string(),
            side: "LH".to_string(),
            plan_qty: 100,
            batch_no: "260202G71".to_string(),
            running_no: "0001".to_string(),
            machine_id: "G7".to_string(),
            shift: 1,
            qr_code: "ARKR-7113800055-260202G71-0001-LH-P992383".to_string(),
        };

        let zpl = generate_zpl(&label);
        
        // ตรวจสอบว่ามีข้อมูลครบ
        assert!(zpl.contains("^XA"));
        assert!(zpl.contains("^XZ"));
        assert!(zpl.contains("Ford"));
        assert!(zpl.contains("LH"));
        assert!(zpl.contains("0001"));
        assert!(zpl.contains("P992383"));
        assert!(zpl.contains("7113800055"));
        assert!(zpl.contains("260202G71"));
        assert!(zpl.contains("ARKR-7113800055-260202G71-0001-LH-P992383"));
    }

    #[test]
    fn test_zpl_qr_code_format() {
        let label = CompleteLabelData {
            prod_id: "TEST123".to_string(),
            item_id: "9999999999".to_string(),
            brand_name: "AEROKLAS".to_string(),
            side: "RH".to_string(),
            plan_qty: 50,
            batch_no: "260414E53".to_string(),
            running_no: "0025".to_string(),
            machine_id: "E5".to_string(),
            shift: 3,
            qr_code: "ARKR-9999999999-260414E53-0025-RH-TEST123".to_string(),
        };

        let zpl = generate_zpl(&label);
        
        assert!(zpl.contains("AEROKLAS"));
        assert!(zpl.contains("RH"));
        assert!(zpl.contains("0025"));
        assert!(zpl.contains("ARKR-9999999999-260414E53-0025-RH-TEST123"));
    }
}
#[test]
#[ignore]
fn print_sample_zpl() {
    let label = CompleteLabelData {
        prod_id: "P992383".to_string(),
        item_id: "7113800055".to_string(),
        brand_name: "Ford".to_string(),
        side: "LH".to_string(),
        plan_qty: 100,
        batch_no: "260202G71".to_string(),
        running_no: "0001".to_string(),
        machine_id: "G7".to_string(),
        shift: 1,
        qr_code: "ARKR-7113800055-260202G71-0001-LH-P992383".to_string(),
    };

    let zpl = generate_zpl(&label);
    println!("\n{}\n", "=".repeat(60));
    println!("ZPL Code Sample:");
    println!("{}", "=".repeat(60));
    println!("{}", zpl);
    println!("{}\n", "=".repeat(60));
}