use axum::{
    headers::{ContentType, Header, HeaderValue},
    response::Html,
    routing::get,
    Router, TypedHeader,
};
use quux::App;
use shared::{render_to_string, Component};
use std::net::SocketAddr;

mod tests;

async fn root() -> Html<String> {
    render_to_string(App::init(())).into()
}

async fn wasm() -> (TypedHeader<ContentType>, &'static [u8]) {
    (
        TypedHeader(
            ContentType::decode(&mut [&HeaderValue::from_static("application/wasm")].into_iter())
                .unwrap(),
        ),
        include_bytes!("../assets/quux_bg.wasm"),
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/assets/quux_bg.wasm", get(wasm));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
