use crate::data::Term;
use quux::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Term,
    Definition,
}

impl Side {
    pub const fn flip(self) -> Self {
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
    is_hidden: Store<bool>,
}

impl Flashcard {
    pub fn new(term: Term, side: Store<Side>) -> Self {
        Self {
            term,
            side,
            flipped: Store::new(false),
            is_hidden: Store::new(false),
        }
    }
}

impl Component for Flashcard {
    fn render(self) -> impl Item {
        let term_hidden = Store::new(true);
        // let side = self.side.clone();
        let definition_hidden = Store::new(false);
        self.side.on_change({
            let term_hidden = term_hidden.clone();
            let definition_hidden = definition_hidden.clone();
            move |_, side| {
                if side == &Side::Term {
                    term_hidden.set(false);
                    definition_hidden.set(true);
                } else {
                    term_hidden.set(true);
                    definition_hidden.set(false);
                }
            }
        });

        article()
            .class("grid place-items-center gap-4 text-center")
            .reactive_class("hidden", self.is_hidden)
            .child(div().class("relative min-w-[60ch] min-h-[40ch]")
                .child(
                    div()
                        .class("card bg-base-200 shadow definition absolute top-0 left-0 w-full h-full grid place-items-center transition-[opacity,transform] duration-300")
                        .reactive_class("flashcard-hidden", term_hidden)
                        .child(
                            div()
                                .class("card-body")
                                .child(
                                    p().text(self.term.term)
                                )
                        )
                )
                .child(
                    div()
                        .class("card bg-base-200 shadow definition absolute top-0 left-0 w-full h-full grid place-items-center transition-[opacity,transform] duration-300 flashcard-hidden")
                        .reactive_class("flashcard-hidden", definition_hidden)
                        .child(
                            div()
                                .class("card-body")
                                .child(
                                    p().text(self.term.definition)
                                )
                        )
                )
            )
    }
}
