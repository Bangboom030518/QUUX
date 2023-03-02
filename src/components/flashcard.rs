use crate::QUUXComponentEnum;
use confidence_rating::ConfidenceRating;
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

pub struct Props {
    pub term: String,
    pub definition: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Flashcard {
    term: String,
    definition: String,
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
    type Props = Props;
    type ComponentEnum = QUUXComponentEnum;

    fn init(props: Self::Props) -> Self {
        let Props { term, definition } = props;
        Self {
            term,
            definition,
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
                            p {{ self.term }}
                        }
                    }
                    div(
                        class = "card bg-base-200 shadow definition absolute top-0 left-0 w-full h-full grid place-items-center transition-[opacity,transform] duration-300 flashcard-hidden",
                        class:active-when = (&self.side, |side| side != Side::Definition, "flashcard-hidden")
                    ) {
                        div(class = "card-body") {
                            p {{ self.definition }}
                        }
                    }
                }
                // button(class = "btn", on:click = {
                //     let side = self.side.clone();
                //     let flipped = self.flipped.clone();
                //     move || {
                //         let previous = *side.get();
                //         side.set(previous.flip());
                //         if !*flipped.get() {
                //             flipped.set(true);
                //             confidence_rating.show();
                //         }
                //     }
                // }) {{"flip"}}
                // @ConfidenceRating: confidence_rating
            }
        }
    }
}
