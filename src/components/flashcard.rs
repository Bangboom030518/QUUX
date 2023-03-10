use super::set::Term;
use crate::QUUXComponentEnum;
use quux::prelude::*;
use quux::{Component, Store};
use serde::{Deserialize, Serialize};

pub mod confidence_rating;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Term,
    Definition,
}

impl Side {
    #[cfg(target_arch = "wasm32")]
    const fn flip(self) -> Self {
        match self {
            Self::Term => Self::Definition,
            Self::Definition => Self::Term,
        }
    }
}

impl Default for Side {
    fn default() -> Self {
        Self::Term
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flashcard {
    term: Term,
    side: Store<Side>,
    flipped: Store<bool>,
    is_visible: Store<bool>,
}

impl Flashcard {
    #[cfg(target_arch = "wasm32")]
    pub fn flip(&self) {
        let previous = *self.side.get();
        self.side.set(previous.flip());
    }

    pub fn show(&self) {
        self.is_visible.set(true);
    }

    pub fn hide(&self) {
        self.is_visible.set(false);
    }
}

impl Component for Flashcard {
    type Props = Term;
    type ComponentEnum = QUUXComponentEnum;

    fn init(term: Term) -> Self {
        Self {
            term,
            side: Store::new(Side::Term),
            flipped: Store::new(false),
            is_visible: Store::new(true),
        }
    }

    fn render(
        &self,
        context: quux::RenderContext<Self::ComponentEnum>,
    ) -> quux::RenderData<Self::ComponentEnum> {
        // let confidence_rating: ConfidenceRating;
        view! {
            article(class = "grid place-items-center gap-4 text-center", class:active-when = (&self.is_visible, |visible: bool| !visible, "hidden")) {
                div(class = "relative min-w-[60ch] min-h-[40ch]") {
                    div(
                        class = "card bg-base-200 shadow term absolute top-0 left-0 w-full h-full grid place-items-center transition-[opacity,transform] duration-300",
                        class:active-when = (&self.side, |side| side != Side::Term, "flashcard-hidden")
                    ) {
                        div(class = "card-body") {
                            p {{ self.term.term }}
                        }
                    }
                    div(
                        class = "card bg-base-200 shadow definition absolute top-0 left-0 w-full h-full grid place-items-center transition-[opacity,transform] duration-300 flashcard-hidden",
                        class:active-when = (&self.side, |side| side != Side::Definition, "flashcard-hidden")
                    ) {
                        div(class = "card-body") {
                            p {{ self.term.definition }}
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for Flashcard {
    fn drop(&mut self) {
        quux::console_log!(
            "My last words are '{:?}'. I hope the afterlife is better than this has been.",
            self.term
        );
    }
}
