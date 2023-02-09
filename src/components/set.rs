use crate::QUUXComponentEnum;
use super::flashcard::confidence_rating::ConfidenceRating;
use super::flashcard::Flashcard;
use quux::{Component};
use quux::prelude::*;
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

    fn render(&self, context: quux::RenderContext<Self::ComponentEnum>) -> quux::RenderData<Self::ComponentEnum> {
        view! {
            div(magic = true) {
                for Term { term, definition } in self.terms.clone().into_iter() {
                    @Flashcard(term = term, definition = definition)
                }
            }
        }  
    }
}
