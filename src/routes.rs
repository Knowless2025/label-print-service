use axum::{Router, routing::{get, post}};
use crate::handlers::print_label;
use crate::AppState;

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/print/label", post(print_label))
}
