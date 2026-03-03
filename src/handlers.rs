use axum::{
    body::Bytes,
    extract::State,
    http::{StatusCode, header},
    Json,
    response::{IntoResponse, Response},
};
use reqwest::Client;
use serde_json::json;

use crate::{
    AppState,
    models::{PrintRequest, LabelRequest},
    zpl::generate_zpl,
};

fn bad_request(msg: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(json!({ "error": msg })),
    ).into_response()
}

fn conflict(msg: &str) -> Response {
    (
        StatusCode::CONFLICT,
        Json(json!({ "error": msg })),
    ).into_response()
}

// 1. API: ตัดยอดและอัปเดตข้อมูลการพิมพ์ใน SQLite
pub async fn print_label(
    State(state): State<AppState>,
    Json(req): Json<PrintRequest>,
) -> Response {

    if req.barcode.trim().is_empty() {
        return bad_request("barcode is required");
    }

    let mut db = state.db.lock().unwrap();

    if let Err(_) = db.get_or_create_plan(
        &req.barcode,
        &req.part_no,
        &req.side,
        req.plan_qty,
    ) {
        return conflict("PLAN_CREATE_FAILED");
    }

    match db.consume_next_running(&req.barcode) {
        Ok(running_no) => {
            Json(json!({
                "status": "PRINT_ACCEPTED",
                "barcode": req.barcode,
                "running_no": running_no
            }))
            .into_response()
        }
        Err(_) => conflict("PLAN_QTY_EXCEEDED"),
    }
}

// 2. API: ทดสอบแปลง Data เป็น ZPL Format (Text)
pub async fn preview_label(
    Json(req): Json<LabelRequest>,
) -> Response {
    let zpl_data = generate_zpl(&req);
    (StatusCode::OK, zpl_data).into_response()
}

// 3. API: ทดสอบสร้างรูป PNG ผ่าน Labelary (เห็นภาพฉลากจริง)
pub async fn preview_label_image(
    Json(req): Json<LabelRequest>,
) -> Response {
    let zpl_string = generate_zpl(&req);
    let client = Client::new();
    
    // ตั้งค่าความละเอียด 8 dpmm (203 dpi) ขนาด 4x6 นิ้ว
    // เปลี่ยนจาก 4x6 เป็น 2x1.5 นิ้ว
    let url = "http://api.labelary.com/v1/printers/8dpmm/labels/2x1.5/0/";

    let response = match client.post(url).body(zpl_string).send().await {
        Ok(res) => res,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to connect to Labelary: {}", e)).into_response(),
    };

    if response.status().is_success() {
        let image_bytes: Bytes = match response.bytes().await {
            Ok(b) => b,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to read image data: {}", e)).into_response(),
        };

        (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "image/png")],
            image_bytes,
        ).into_response()
    } else {
        let error_text = response.text().await.unwrap_or_default();
        (StatusCode::BAD_REQUEST, format!("Labelary Error: {}", error_text)).into_response()
    }
}