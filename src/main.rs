#![feature(more_qualified_paths)]
use axum::routing::get_service;
use axum::{
    headers::{ContentType, Header, HeaderValue},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router, TypedHeader,
};
use html::view;
use shared::QUUXInitialise;
use shared::{init_app, Component, Render, RenderData, Store};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{services::ServeFile, ServiceBuilderExt};

mod tests;

struct App<'a> {
    count: Store<'a, u32>,
}

impl<'a> Component for App<'a> {
    type Props = ();

    fn init(_props: Self::Props) -> Self {
        Self {
            count: Store::new(0),
        }
    }
}

impl<'a> Render for App<'a> {
    fn render(&self) -> RenderData {
        view! {
            html(lang="en") {
                head {}
                body {
                    button {
                        { self.count }
                    }
                    @QUUXInitialise
                }
            }
        }
    }
}

async fn root() -> Html<String> {
    init_app(App::init(())).into()
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
    // let middleware = ServiceBuilder::new()
    //     .layer(ServeFile::new("assets/quux_bg.wasm"))
    //     .layer(TimeoutLayer::new(Duration::from_secs(10)))
    //     .map_response_body(axum::body::boxed::<String>)
    //     .insert_response_header_if_not_present(
    //         header::CONTENT_TYPE,
    //         HeaderValue::from_static("application/octet-stream"),
    //     );

    let app = Router::new()
        .route("/", get(root))
        .route("/assets/quux_bg.wasm", get(wasm));
    // .route(
    //     "/assets/quux_bg.wasm",
    //     get_service(ServeFile::new("assets/quux_bg.wasm")).handle_error(handle_error),
    // );
    // .layer(ServiceBuilder::new().layer(ServeFile::new("assets/quux_bg.wasm")).layer(State {}));
    // .layer(tower::make::Shared::new(ServeFile::new(
    //     "assets/quux_bg.wasm",
    // )));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

// Layer<MapResponseBody<Route<_>, fn(String) -> http_body::combinators::box_body::UnsyncBoxBody<axum::body::Bytes, axum::Error> {axum::body::boxed::<String>}>>
