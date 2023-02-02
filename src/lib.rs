#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]
use components::{flashcard, set};
use quux::{Component, ComponentEnum, QUUXInitialise, Store};
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
    App::init_as_root::<QUUXComponentEnum>();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
enum QUUXComponentEnum {
    App(App),
    Flashcard(flashcard::Flashcard),
    QUUXInitialise(QUUXInitialise),
}

impl ComponentEnum for QUUXComponentEnum {
    fn render(&self, context: quux::RenderContext<Self>) -> quux::RenderData<Self> {
        match self {
            Self::App(component) => component.render(context),
            Self::Flashcard(component) => component.render(context),
            Self::QUUXInitialise(component) => component.render(context),
        }
    }
}

impl From<QUUXInitialise> for QUUXComponentEnum {
    fn from(value: QUUXInitialise) -> Self {
        Self::QUUXInitialise(value)
    }
}

impl From<App> for QUUXComponentEnum {
    fn from(value: App) -> Self {
        Self::App(value)
    }
}

impl From<flashcard::Flashcard> for QUUXComponentEnum {
    fn from(value: flashcard::Flashcard) -> Self {
        Self::Flashcard(value)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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

    fn render<T: ComponentEnum>(&self, context: quux::RenderContext<T>) -> quux::RenderData<T> {
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
                    @flashcard::Flashcard(term = "a".to_string(), definition = "b".to_string())
                    @set::Set(terms = vec![set::Term::new("0", "1"), set::Term::new("2", "3")])
                    @QUUXInitialise(init_script_content = include_str!("../dist/init.js"))
                }
            }
        }
    }
}
