#![warn(clippy::pedantic, clippy::nursery)]
#![cfg(not(target_arch = "wasm32"))]
use axum::{
    headers::{ContentType, Header, HeaderValue},
    response::Html,
    routing::get,
    Router, TypedHeader,
};
use quux::Component;
use quuxlet::App;
use std::net::SocketAddr;

async fn root() -> Html<String> {
    App::init(()).render_to_string().into()
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
    let app = Router::new()
        .route("/", get(root))
        // TODO rename
        .route("/dist/quuxlet_bg.wasm", get(wasm));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on http://{address}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
