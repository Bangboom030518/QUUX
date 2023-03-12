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
    QUUXComponentEnum::init_app().unwrap();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum QUUXComponentEnum {
    App(App),
    Flashcard(flashcards::Flashcard),
    QUUXInitialise(QUUXInitialise<Self>),
    Flashcards(Flashcards),
    ConfidenceRating(flashcards::ConfidenceRating),
}

impl component::Enum for QUUXComponentEnum {
    fn render(&self, context: render::Context<Self>) -> render::Output<Self> {
        match self {
            Self::App(component) => component.render(context),
            Self::Flashcard(component) => component.render(context),
            Self::QUUXInitialise(component) => component.render(context),
            Self::Flashcards(component) => component.render(context),
            Self::ConfidenceRating(component) => component.render(context),
        }
    }
}

impl From<QUUXInitialise<Self>> for QUUXComponentEnum {
    fn from(value: QUUXInitialise<Self>) -> Self {
        Self::QUUXInitialise(value)
    }
}

impl TryFrom<QUUXComponentEnum> for QUUXInitialise<QUUXComponentEnum> {
    type Error = ();

    fn try_from(value: QUUXComponentEnum) -> Result<Self, Self::Error> {
        if let QUUXComponentEnum::QUUXInitialise(component) = value {
            Ok(component)
        } else {
            Err(())
        }
    }
}

impl From<App> for QUUXComponentEnum {
    fn from(value: App) -> Self {
        Self::App(value)
    }
}

impl TryFrom<QUUXComponentEnum> for App {
    type Error = ();

    fn try_from(value: QUUXComponentEnum) -> Result<Self, Self::Error> {
        if let QUUXComponentEnum::App(component) = value {
            Ok(component)
        } else {
            Err(())
        }
    }
}

impl From<flashcards::Flashcard> for QUUXComponentEnum {
    fn from(value: flashcards::Flashcard) -> Self {
        Self::Flashcard(value)
    }
}

impl TryFrom<QUUXComponentEnum> for flashcards::Flashcard {
    type Error = ();

    fn try_from(value: QUUXComponentEnum) -> Result<Self, Self::Error> {
        if let QUUXComponentEnum::Flashcard(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl From<Flashcards> for QUUXComponentEnum {
    fn from(value: Flashcards) -> Self {
        Self::Flashcards(value)
    }
}

impl TryFrom<QUUXComponentEnum> for Flashcards {
    type Error = ();

    fn try_from(value: QUUXComponentEnum) -> Result<Self, Self::Error> {
        if let QUUXComponentEnum::Flashcards(component) = value {
            Ok(component)
        } else {
            Err(())
        }
    }
}

impl From<flashcards::ConfidenceRating> for QUUXComponentEnum {
    fn from(value: flashcards::ConfidenceRating) -> Self {
        Self::ConfidenceRating(value)
    }
}

impl TryFrom<QUUXComponentEnum> for flashcards::ConfidenceRating {
    type Error = ();

    fn try_from(value: QUUXComponentEnum) -> Result<Self, Self::Error> {
        if let QUUXComponentEnum::ConfidenceRating(component) = value {
            Ok(component)
        } else {
            Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rating {
    Terrible,
    Bad,
    Ok,
    Good,
    Perfect,
}

impl Default for Rating {
    fn default() -> Self {
        Self::Ok
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct App {
    set: Set,
}

impl Component for App {
    type Props = flashcards::Set;
    type ComponentEnum = QUUXComponentEnum;

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
