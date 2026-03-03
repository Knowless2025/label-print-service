use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PrintRequest {
    pub barcode: String,
    pub part_no: String,
    pub side: String,
    pub plan_qty: i32,
}

// เพิ่ม Serialize และ Clone เข้ามาตรงนี้
#[derive(Debug, Deserialize, Serialize, Clone)] 
pub struct LabelRequest {
    pub customer_code: String,
    pub brand: String,
    pub side: String,
    pub part_no: String,
    pub serial_no: String,
    pub barcode_main: String,
    pub barcode_sub: Option<String>,
    pub plan_qty: i32, // เพิ่มจำนวนผลิตเข้ามาด้วย
}