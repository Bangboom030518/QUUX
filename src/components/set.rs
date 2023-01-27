use super::flashcard::confidence_rating::ConfidenceRating;
use super::flashcard::Flashcard;
use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, Store};

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct Set {
    terms: Vec<Term>,
}

impl Component for Set {
    type Props = Props;

    fn init(Props { terms }: Props) -> Self {
        Self { terms }
    }

    fn render(&self, context: shared::RenderContext) -> shared::RenderData {
        view! {
            div(magic = true) {
                for Term { term, definition } in self.terms.clone().into_iter() {
                    // div {
                    //     {format!("{term}={definition}")}
                    // }
                    @Flashcard(term = term, definition = definition)
                }
            }
        }
    }
}
