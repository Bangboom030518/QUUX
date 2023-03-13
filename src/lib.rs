// TODO: should `render` gobble gobble gobble?

#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]

use components::{flashcards, Flashcards};
pub use flashcards::Set;
use quux::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod components;

/// # Panics
/// This function will panic if it's unable to retrieve and parse the tree sent by the server
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init_app() {
    ComponentEnum::init_app().unwrap();
}

init_components!(
    App,
    Flashcards,
    flashcards::Flashcard,
    flashcards::ConfidenceRating
);

#[cfg(not(target_arch = "wasm32"))]
pub async fn server_error(error: tower::BoxError) -> (axum::http::StatusCode, String) {
    use axum::http::StatusCode;
    if error.is::<tower::timeout::error::Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Uh oh. Something went wrong — sucks to be you.".to_string(),
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct App {
    set: Set,
}

#[cfg(not(target_arch = "wasm32"))]
impl App {
    pub async fn new(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        set_id: &str,
    ) -> Result<Self, (axum::http::StatusCode, String)> {
        use axum::http::StatusCode;
        match Set::fetch(pool, set_id).await {
            Ok(set) => Ok(Self::init(set)),
            Err(error) => Err(match error {
                sqlx::Error::RowNotFound => (StatusCode::NOT_FOUND, "Set not found :(".to_string()),
                _ => server_error(Box::new(error)).await,
            }),
        }
    }
}

impl Component for App {
    type Props = flashcards::Set;
    type ComponentEnum = ComponentEnum;

    fn init(set: Set) -> Self {
        Self { set }
    }

    fn render(
        &self,
        context: render::Context<Self::ComponentEnum>,
    ) -> render::Output<Self::ComponentEnum> {
        view! {
            context,
            html(lang="en") {
                head {
                    meta(charset="UTF-8") {}
                    meta(http-equiv="X-UA-Compatible", content="IE=edge") {}
                    meta(name="viewport", content="width=device-width, initial-scale=1.0") {}
                    style {
                        { include_str!("../dist/output.css") }
                    }
                    title {{ "Document" }}
                }
                body {
                    h1 {{ "Welcome to Quuxlet" }}
                    @Flashcards(self.set.terms.clone())
                    @QUUXInitialise<Self::ComponentEnum>(include_str!("../dist/init.js"))
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl axum::response::IntoResponse for App {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html::from(self.render_to_string()).into_response()
    }
}
