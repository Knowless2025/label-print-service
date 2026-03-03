use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PrintRequest {
    pub barcode: String,
    pub part_no: String,
    pub side: String,
    pub plan_qty: i32,
}

#[derive(Debug, Deserialize)]
pub struct LabelRequest {
    pub customer_code: String,
    pub brand: String,
    pub side: String,
    pub part_no: String,
    pub serial_no: String,
    pub barcode_main: String,
    pub barcode_sub: Option<String>,
}