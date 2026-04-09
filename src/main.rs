mod db;
mod models;
mod qr;   
// mod handlers;  // ← Comment ไว้ก่อน รอ Step 6
mod zpl;       // ← Comment ไว้ก่อน รอ Step 5

use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use crate::db::init_db;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<db::Database>>,
    // ยังไม่ต้องใช้ตอนนี้ รอ Step 6
    // pub active_label: Arc<Mutex<Option<models::ERPLabelData>>>,
}

#[tokio::main]
async fn main() {
    println!("🚀 Label Print Service (Development Mode)");
    println!("📦 Step 1-3: Database Ready");
    println!("⏳ Waiting for Step 4-7...\n");

    let conn = Connection::open("printlabel.db").expect("failed to open db");
    init_db(&conn).expect("failed to init db");

    let db = db::Database { conn };
    
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Router แบบง่าย รอ Step 6
    let app = Router::new()
        .route("/", get(|| async { "Label Print Service - Development Mode" }))
        .route("/health", get(|| async { "OK" }))
        .layer(cors)
        .with_state(state);

    println!("✅ Server running on http://0.0.0.0:3000");
    println!("📊 Database tables created successfully!\n");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}