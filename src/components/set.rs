use super::flashcard::confidence_rating::ConfidenceRating;
use super::flashcard::Flashcard;
use crate::QUUXComponentEnum;
use quux::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

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
        context: render::Context<Self::ComponentEnum>,
    ) -> render::Output<Self::ComponentEnum> {
        let confidence_rating: ConfidenceRating;
        let flashcards: Rc<RefCell<Vec<Flashcard>>>;
        view! {
            context,
            div(magic= true, class = "grid place-items-center gap-4") {
                div(class = "flashcard-stack") {
                    for term in $self.terms {
                        @Flashcard(term): flashcards
                    }
                }
                button(class = "btn", on:click = {
                    let rating = confidence_rating.get_rating_store();
                    let flashcards = Rc::clone(&flashcards);
                    let terms = self.terms.clone();
                    let confidence_rating = Rc::new(confidence_rating);

                    rating.on_change({
                        let confidence_rating = Rc::clone(&confidence_rating);
                        move |_, _| {
                            terms.pop();
                            confidence_rating.hide();
                        }
                    });

                    move || {
                        let flashcards = flashcards.borrow();
                        let Some(flashcard) = flashcards.last() else {
                            quux::console_log!("No flashcards found");
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
