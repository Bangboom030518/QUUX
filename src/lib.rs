#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]

use components::flashcards;
pub use flashcards::Set;
use quux::{prelude::*, view::SerializedComponent};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod components;
pub mod pages;

#[derive(Serialize, Deserialize)]
pub enum Routes {
    Set(SerializedComponent<pages::Set>),
    ServerError(SerializedComponent<pages::Error>),
}

impl From<SerializedComponent<pages::Set>> for Routes {
    fn from(value: SerializedComponent<pages::Set>) -> Self {
        Self::Set(value)
    }
}

impl From<SerializedComponent<pages::Error>> for Routes {
    fn from(value: SerializedComponent<pages::Error>) -> Self {
        Self::ServerError(value)
    }
}

impl quux::component::Routes for Routes {
    #[client]
    fn render(self) {
        match self {
            Self::Set(set) => {
                set.render();
            }
            Self::ServerError(server_error) => {
                server_error.render();
            }
        };
    }
}

#[server]
impl axum::response::IntoResponse for pages::Set {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html::from(Routes::render_to_string(self)).into_response()
    }
}

#[server]
impl axum::response::IntoResponse for pages::Error {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html::from(Routes::render_to_string(self)).into_response()
    }
}

/// # Panics
/// This function will panic if it's unable to retrieve and parse the tree sent by the server
// #[cfg(target_arch = "wasm32")]
#[client]
#[wasm_bindgen(start)]
pub fn init_app() {
    Routes::init_app().unwrap();
}
