use std::io::Error;

use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct App {
    name: &'static str,
    version: &'static str,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new().route("/", get(root));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await
}

async fn root() -> Json<App> {
    Json(App {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
    })
}
