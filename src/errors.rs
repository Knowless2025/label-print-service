use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub fn bad_request(msg: &str) -> Response {
    (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            error: msg.to_string(),
        }),
    )
        .into_response()
}

pub fn conflict(msg: &str) -> Response {
    (
        StatusCode::CONFLICT,
        Json(ErrorResponse {
            error: msg.to_string(),
        }),
    )
        .into_response()
}
