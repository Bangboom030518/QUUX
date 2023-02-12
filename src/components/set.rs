use super::flashcard::confidence_rating::ConfidenceRating;
use super::flashcard::Flashcard;
use crate::QUUXComponentEnum;
use quux::prelude::*;
use quux::Component;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Term {
    term: String,
    definition: String,
}

impl Term {
    pub fn new(term: &str, definition: &str) -> Self {
        Self {
            term: term.to_string(),
            definition: definition.to_string(),
        }
    }
}

pub struct Props {
    pub terms: Vec<Term>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set {
    terms: Vec<Term>,
}

impl Component for Set {
    type Props = Props;
    type ComponentEnum = QUUXComponentEnum;

    fn init(Props { terms }: Props) -> Self {
        Self { terms }
    }

    fn render(
        &self,
        context: quux::RenderContext<Self::ComponentEnum>,
    ) -> quux::RenderData<Self::ComponentEnum> {
        let mut flashcards: Vec<Flashcard> = Vec::new();
        view! {
            div(magic = true) {
                div {
                    for Term { term, definition } in self.terms.clone().into_iter() {
                        @Flashcard(term = term, definition = definition): flashcards
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
