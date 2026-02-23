mod db;
mod handlers;
mod models;

use axum::{
    routing::post,
    Router,
};
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

    // router
    let app = Router::new()
        .route("/print/label", post(handlers::print_label))
        .with_state(state);

    println!("🚀 Server running on http://0.0.0.0:3000");

use tokio::net::TcpListener;

let listener = TcpListener::bind("0.0.0.0:3000")
    .await
    .expect("failed to bind");

axum::serve(listener, app)
    .await
    .unwrap();

}
