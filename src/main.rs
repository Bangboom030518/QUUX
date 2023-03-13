#![warn(clippy::pedantic, clippy::nursery)]
// TODO: remove?
#![allow(clippy::unused_async)]
#![cfg(not(target_arch = "wasm32"))]

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, State},
    headers::{ContentType, Header, HeaderValue},
    http::StatusCode,
    response::Html,
    routing::get,
    BoxError, Router, TypedHeader,
};
use quux::prelude::*;
use quuxlet::{App, Set};
use sqlx::{Pool, Sqlite};
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;
use tower_http::services::ServeFile;

async fn root() -> Html<String> {
    "
        <h1>Welcome to QUUXlet</h1>
    "
    .to_string()
    .into()
}

async fn not_found() -> (StatusCode, Html<String>) {
    let html = "
        <h1>Page not found!</h1>
    "
    .to_string()
    .into();
    (StatusCode::NOT_FOUND, html)
}

async fn set(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<String>,
) -> Result<App, (StatusCode, String)> {
    App::new(&pool, &id).await
}

#[tokio::main]
async fn main() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://src/database/data.db")
        .await
        .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/set/:set_id", get(set))
        .fallback(not_found)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(quuxlet::server_error))
                .timeout(Duration::from_secs(30)),
        )
        .route_service(
            "/dist/quuxlet_bg.wasm",
            ServeFile::new_with_mime(
                "dist/quuxlet_bg.wasm",
                &"application/wasm".parse::<mime::Mime>().unwrap(),
            ),
        )
        .with_state(pool);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{address}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
