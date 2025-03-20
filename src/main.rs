#![forbid(unsafe_code)]

// External crates for logging functionality
extern crate log;
extern crate pretty_env_logger;

mod database;
mod traits;
mod web_api;

use std::sync::Arc;

use anyhow::Result;
use axum::routing::{get, post};
use log::info;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};
use web_api::{create_build, get_build};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let db = database::BuildsRedisDatabase::default();
    info!("initialized database");

    let static_files = axum::Router::new().route_service(
        "/",
        ServeDir::new("static").not_found_service(ServeFile::new("static/404.html")),
    );

    let builds = axum::Router::new()
        .route("/build/{id}", get(get_build))
        .route("/build/create", post(create_build))
        .with_state(Arc::new(db));

    let app = axum::Router::new().merge(static_files).merge(builds);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    info!("server started");
    axum::serve(listener, app).await?;

    Ok(())
}
