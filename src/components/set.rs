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
            div {
                // for term in terms {
                //     div(class = "lolz") {

                //     }
                //     // @Flashcard(term = term, definition = definition)
                // }
                for (a in b) {
                    c
                }
            }
        }
    }
}
