#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]
use components::{flashcard, set};
use quux::prelude::*;
use quux::{Component, ComponentEnum, QUUXInitialise, Store};
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
pub enum QUUXComponentEnum {
    App(App),
    Flashcard(flashcard::Flashcard),
    QUUXInitialise(QUUXInitialise<Self>),
    Set(set::Set),
    ConfidenceRating(flashcard::confidence_rating::ConfidenceRating),
}

impl ComponentEnum for QUUXComponentEnum {
    fn render(&self, context: quux::RenderContext<Self>) -> quux::RenderData<Self> {
        match self {
            Self::App(component) => component.render(context),
            Self::Flashcard(component) => component.render(context),
            Self::QUUXInitialise(component) => component.render(context),
            Self::Set(component) => component.render(context),
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

impl From<flashcard::Flashcard> for QUUXComponentEnum {
    fn from(value: flashcard::Flashcard) -> Self {
        Self::Flashcard(value)
    }
}

impl TryFrom<QUUXComponentEnum> for flashcard::Flashcard {
    type Error = ();

    fn try_from(value: QUUXComponentEnum) -> Result<Self, Self::Error> {
        if let QUUXComponentEnum::Flashcard(value) = value {
            Ok(value)
        } else {
            Err(())
        }
    }
}

impl From<set::Set> for QUUXComponentEnum {
    fn from(value: set::Set) -> Self {
        Self::Set(value)
    }
}

impl TryFrom<QUUXComponentEnum> for set::Set {
    type Error = ();

    fn try_from(value: QUUXComponentEnum) -> Result<Self, Self::Error> {
        if let QUUXComponentEnum::Set(component) = value {
            Ok(component)
        } else {
            Err(())
        }
    }
}

impl From<flashcard::confidence_rating::ConfidenceRating> for QUUXComponentEnum {
    fn from(value: flashcard::confidence_rating::ConfidenceRating) -> Self {
        Self::ConfidenceRating(value)
    }
}

impl TryFrom<QUUXComponentEnum> for flashcard::confidence_rating::ConfidenceRating {
    type Error = ();

    fn try_from(value: QUUXComponentEnum) -> Result<Self, Self::Error> {
        if let QUUXComponentEnum::ConfidenceRating(component) = value {
            Ok(component)
        } else {
            Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct App {
    count: Store<u32>,
}

impl Component for App {
    type Props = ();
    type ComponentEnum = QUUXComponentEnum;

    fn init(_props: Self::Props) -> Self {
        Self {
            count: Store::new(0),
        }
    }

    fn render(
        &self,
        context: quux::RenderContext<Self::ComponentEnum>,
    ) -> quux::RenderData<Self::ComponentEnum> {
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
                    h1 {{ "Welcome to Quuxlet" }}
                    @set::Set(vec![set::Term::new("term 1", "definition 1"), set::Term::new("term 2", "definition 2"), set::Term::new("term 3", "definition 3")])
                    @QUUXInitialise<Self::ComponentEnum>(include_str!("../dist/init.js"))
                }
            }
        }
    }
}
