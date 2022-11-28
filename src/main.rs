use html::view;
use shared::QUUXInitialise;
use shared::{init_app, Component, Render, RenderData, Store};
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::{IntoResponse, Html},
    Json, Router,
};
use std::net::SocketAddr;

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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
