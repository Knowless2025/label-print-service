use serde::{Deserialize, Serialize};

// ========================================
// Request จาก ERP AX4 (Stage Data)
// ========================================
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ERPLabelData {
    pub prod_id: String,              // Production Order ID (P992383)
    pub item_id: String,              // Part Number (7113800055)
    pub brand_name: String,           // Brand (Ford, AEROKLAS, etc.)
    pub side: String,                 // LH/RH
    pub plan_qty: i32,                // Plan Quantity
}

// ========================================
// Request สำหรับเริ่มผลิต (จาก Web UI)
// ========================================
#[derive(Debug, Deserialize)]
pub struct StartProductionRequest {
    pub prod_id: String,              // Production Order ID
    pub item_id: String,              // Part Number
    pub brand_name: String,           // Brand Name
    pub side: String,                 // LH/RH
    pub plan_qty: i32,                // Plan Quantity
    pub machine_id: String,           // D9, E2, G7 (พนักงานเลือก)
    pub shift: i32,                   // 1, 2, 3 (พนักงานเลือก)
}

// ========================================
// ข้อมูลฉลากสมบูรณ์ (สำหรับพิมพ์)
// ========================================
#[derive(Debug, Serialize, Clone)]
pub struct CompleteLabelData {
    // จาก ERP
    pub prod_id: String,              // P992383
    pub item_id: String,              // 7113800055
    pub brand_name: String,           // Ford
    pub side: String,                 // LH
    pub plan_qty: i32,                // 100
    
    // Generate โดยระบบ
    pub batch_no: String,             // 260202G71
    pub running_no: String,           // 0001 (4 digits)
    pub machine_id: String,           // G7
    pub shift: i32,                   // 1
    pub qr_code: String,              // ARKR-7113800055-260202G71-0001-LH-P992383
}

// ========================================
// Response หลังพิมพ์สำเร็จ
// ========================================
#[derive(Serialize)]
pub struct PrintResponse {
    pub status: String,               // "PRINT_SUCCESS"
    pub prod_id: String,              // P992383
    pub running_no: String,           // 0001
    pub batch_no: String,             // 260202G71
    pub remaining: i32,               // จำนวนที่เหลือ
    pub qr_code: String,              // ARKR-...
}

// ========================================
// Response สถานะการผลิต (สำหรับ UI)
// ========================================
#[derive(Serialize)]
pub struct ProductionStatus {
    pub prod_id: String,              // P992383
    pub item_id: String,              // 7113800055
    pub brand_name: String,           // Ford
    pub side: String,                 // LH
    pub batch_no: String,             // 260202G71
    pub current_running: i32,         // 15
    pub plan_qty: i32,                // 100
    pub machine_id: String,           // G7
    pub shift: i32,                   // 1
    pub status: String,               // ACTIVE, PAUSED, COMPLETED
}