use axum::{
    extract::State,
    Json,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{
    AppState,
    models::PrintRequest,
};

fn bad_request(msg: &str) -> Response {
    (
        axum::http::StatusCode::BAD_REQUEST,
        Json(json!({ "error": msg })),
    ).into_response()
}

fn conflict(msg: &str) -> Response {
    (
        axum::http::StatusCode::CONFLICT,
        Json(json!({ "error": msg })),
    ).into_response()
}

pub async fn print_label(
    State(state): State<AppState>,
    Json(req): Json<PrintRequest>,
) -> Response {

    if req.barcode.trim().is_empty() {
        return bad_request("barcode is required");
    }

    let mut db = state.db.lock().unwrap();

    // ensure plan exists
    if let Err(_) = db.get_or_create_plan(
        &req.barcode,
        &req.part_no,
        &req.side,
        req.plan_qty,
    ) {
        return conflict("PLAN_CREATE_FAILED");
    }

    // consume running number
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
