#![warn(clippy::pedantic, clippy::nursery)]
// TODO: remove?
#![allow(clippy::unused_async)]
#![cfg(not(target_arch = "wasm32"))]

use axum::{
    error_handling::HandleErrorLayer,
    extract::{Path, State},
    http::{StatusCode, Request},
    response::Html,
    routing::get,
    body::Body,
    Router,
};
use quux::prelude::*;
use quuxlet::pages::{Error, Set, Create};
use sqlx::{Pool, Sqlite};
use std::{net::SocketAddr, time::Duration};
use tower::ServiceBuilder;
use tower_http::services::ServeFile;

#[axum::debug_handler]
async fn root() -> Html<String> {
    "
        <h1>Welcome to QUUXlet</h1>
    "
    .to_string()
    .into()
}

#[axum::debug_handler]
async fn not_found(request: Request<Body>) -> Error {
    Error::PageNotFound { uri: request.uri().to_string() }
}

#[axum::debug_handler]
async fn create() -> Create {
    Create
}

#[axum::debug_handler]
async fn set(
    State(pool): State<Pool<Sqlite>>,
    Path(id): Path<String>,
) -> Result<Set, quuxlet::pages::Error> {
    Set::new(&pool, &id).await
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
        .route("/create", get(create))
        .fallback(not_found)
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: tower::BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        return quuxlet::pages::Error::Timeout;
                    }

                    quuxlet::pages::Error::Internal {
                        message: error.to_string(),
                    }
                }))
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
