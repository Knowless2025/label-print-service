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
    // เพิ่มห้องพักข้อมูลชั่วคราวตรงนี้
    pub active_label: Arc<Mutex<Option<models::LabelRequest>>>, 
}

#[tokio::main]
async fn main() {
    let conn = Connection::open("printlabel.db").expect("failed to open db");
    init_db(&conn).expect("failed to init db");

    let db = db::Database { conn };
    
    // ตั้งค่า state ตอนเริ่มโปรแกรมให้ห้องพักข้อมูลว่างเปล่า (None)
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
        active_label: Arc::new(Mutex::new(None)),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(handlers::serve_ui))
        .route("/print/label", post(handlers::print_label))
        .route("/print/preview", post(handlers::preview_label))
        .route("/print/preview-image", post(handlers::preview_label_image))
        // เพิ่ม API 2 เส้นใหม่ สำหรับรับ-ส่งข้อมูลระหว่าง Hoppscotch กับ Web UI
        .route("/api/stage", post(handlers::stage_label)) 
        .route("/api/current", get(handlers::get_staged_label))
        .layer(cors)
        .with_state(state);

    println!("🚀 Server running on http://0.0.0.0:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}