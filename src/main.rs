#![warn(clippy::pedantic, clippy::nursery)]
#![cfg(not(target_arch = "wasm32"))]
use axum::{
    extract::{Path, State},
    headers::{ContentType, Header, HeaderValue},
    response::Html,
    routing::get,
    Router, TypedHeader,
};
use quux::prelude::*;
use quuxlet::{App, Set};
use sqlx::{Pool, Sqlite};
use std::net::SocketAddr;

async fn root() -> Html<String> {
    "
        <h1>Welcome to QUUXlet</h1>
    "
    .to_string()
    .into()
}

async fn not_found() -> Html<String> {
    "
        <h1>Error: not found!</h1>
    "
    .to_string()
    .into()
}

async fn set(State(pool): State<Pool<Sqlite>>, Path(id): Path<String>) -> Html<String> {
    App::init(
        Set::fetch(&pool, &id)
            .await
            .unwrap_or_else(|_| todo!("handle db error!")),
    )
    .render_to_string()
    .into()
}

async fn wasm() -> (TypedHeader<ContentType>, &'static [u8]) {
    (
        TypedHeader(
            ContentType::decode(&mut [&HeaderValue::from_static("application/wasm")].into_iter())
                .unwrap(),
        ),
        include_bytes!("../dist/quuxlet_bg.wasm"),
    )
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
        .route("/dist/quuxlet_bg.wasm", get(wasm))
        .with_state(pool);

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{address}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
