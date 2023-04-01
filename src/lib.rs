#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]

use components::flashcards;
pub use flashcards::Set;
use quux::{prelude::*, view::SerializedComponent};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod components;
pub mod pages;

routes!(pages::Set, pages::Error, pages::Create, pages::Index);

#[server]
impl warp::Reply for pages::Set {
    fn into_response(self) -> warp::reply::Response {
        warp::reply::html(Routes::render_to_string(self)).into_response()
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

#[server]
impl axum::response::IntoResponse for pages::Create {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html::from(Routes::render_to_string(self)).into_response()
    }
}

#[server]
impl axum::response::IntoResponse for pages::Index {
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
