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
pub mod pages;

init_components!(
    pages::Set,
    pages::ServerError,
    Flashcards,
    flashcards::Flashcard,
    flashcards::ConfidenceRating
);

/// # Panics
/// This function will panic if it's unable to retrieve and parse the tree sent by the server
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init_app() {
    ComponentEnum::init_app().unwrap();
}
