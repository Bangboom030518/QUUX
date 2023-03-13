// TODO: should `render` gobble gobble gobble?

#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]
use components::{flashcards, Flashcards};
use quux::prelude::*;
use serde::{Deserialize, Serialize};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub use flashcards::Set;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct App {
    set: Set,
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
