mod db;
mod errors;
mod handlers;
mod models;
mod routes;
mod zpl;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::db::init_db;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<db::Database>>,
}

#[tokio::main]
async fn main() {
    // เปิด DB
    let conn = Connection::open("printlabel.db")
        .expect("failed to open db");

    // init schema
    init_db(&conn).expect("failed to init db");

    let db = db::Database { conn };
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    // ตั้งค่า CORS ให้อนุญาตทุกการเชื่อมต่อ (แก้ปัญหา Network Error ใน Hoppscotch)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // ประกาศ Route ทั้งหมด
    let app = Router::new()
        .route("/", get(|| async { "🟢 Label Print Service is Running!" }))
        .route("/print/label", post(handlers::print_label))
        .route("/print/preview", post(handlers::preview_label))
        .route("/print/preview-image", post(handlers::preview_label_image))
        .layer(cors) // เปิดใช้งาน CORS
        .with_state(state);

    println!("🚀 Server running on http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    axum::serve(listener, app)
        .await
        .unwrap();
}