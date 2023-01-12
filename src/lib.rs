#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]
use components::flashcard;
use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, QUUXInitialise, Store};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
mod components;

/// # Panics
/// This function will panic if it's unable to retrieve and parse the tree sent by the server
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init_app() {
    App::init_as_root();
}

#[derive(Serialize, Deserialize)]
pub struct App {
    count: Store<u32>,
}

impl Component for App {
    type Props = ();

    fn init(_props: Self::Props) -> Self {
        Self {
            count: Store::new(0),
        }
    }

    fn render(&self, context: shared::RenderContext) -> shared::RenderData {
        view! {
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
                    h1 {
                        { "Welcome to Quuxlet" }
                    }
                    @flashcard::Flashcard(term = "a", definition = "b")
                    @QUUXInitialise(init_script_content = include_str!("../dist/init.js"))
                }
            }
        }
    }
}
