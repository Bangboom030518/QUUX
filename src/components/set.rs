use super::flashcard::confidence_rating::ConfidenceRating;
use super::flashcard::Flashcard;
use crate::QUUXComponentEnum;
use quux::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Term {
    pub term: String,
    pub definition: String,
}

impl Term {
    pub fn new(term: &str, definition: &str) -> Self {
        Self {
            term: term.to_string(),
            definition: definition.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set {
    terms: store::List<Term>,
}

impl Component for Set {
    type Props = Vec<Term>;
    type ComponentEnum = QUUXComponentEnum;

    fn init(terms: Self::Props) -> Self {
        Self {
            terms: store::List::new(terms),
        }
    }

    fn render(
        &self,
        context: quux::RenderContext<Self::ComponentEnum>,
    ) -> quux::RenderData<Self::ComponentEnum> {
        let confidence_rating: ConfidenceRating;
        let flashcards: Vec<Flashcard>;
        view! {
            div(magic= true, class = "grid place-items-center gap-4") {
                div(class = "flashcard-stack") {
                    for term in $self.terms {
                        @Flashcard(term): flashcards
                    }
                }
                button(class = "btn", on:click = {
                    let rating = confidence_rating.get_rating_store();
                    let terms = self.terms.clone();
                    rating.on_change(move |_, _| {
                        terms.pop();
                    });
                    move || {
                        let Some(flashcard) = flashcards.last() else {
                            console_log!("No flashcards found");
                            return
                        };
                        flashcard.flip();
                        confidence_rating.show();
                    }
                }) {{"flip"}}
                @ConfidenceRating: confidence_rating
            }
        }
    }
}
