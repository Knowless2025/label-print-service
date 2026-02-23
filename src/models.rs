use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PrintRequest {
    pub barcode: String,
    pub part_no: String,
    pub side: String,
    pub plan_qty: i32,
}
