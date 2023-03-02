use super::flashcard::confidence_rating;
use super::flashcard::confidence_rating::ConfidenceRating;
use super::flashcard::Flashcard;
use crate::QUUXComponentEnum;
use quux::prelude::*;
use quux::Component;
use quux::Store;
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
    current_index: Store<usize>,
}

impl Component for Set {
    type Props = Props;
    type ComponentEnum = QUUXComponentEnum;

    fn init(Props { terms }: Props) -> Self {
        Self {
            terms,
            current_index: Store::new(0),
        }
    }

    fn render(
        &self,
        context: quux::RenderContext<Self::ComponentEnum>,
    ) -> quux::RenderData<Self::ComponentEnum> {
        let confidence_rating: ConfidenceRating;
        let flashcards: Vec<Flashcard>;
        // TODO: accept props expr instead of weird attributes
        view! {
            div(magic = true, class = "grid place-items-center gap-4") {
                div(class = "stack") {
                    for Term { term, definition } in self.terms.clone().into_iter() {
                        @Flashcard(term = term, definition = definition): flashcards
                    }
                }
                button(class = "btn", on:click = {
                    let current_index = self.current_index.clone();
                    let rating = confidence_rating.get_rating_store();

                    let flashcards = Rc::new(flashcards);

                    rating.on_change({
                        let current_index = self.current_index.clone();

                        let flashcards = Rc::clone(&flashcards);

                        move |_, new_rating| {
                            let old_index = *current_index.get();
                            flashcards.get(old_index).unwrap().hide();
                            current_index.set(old_index + 1);
                        }
                    });
                    let flashcards = Rc::clone(&flashcards);
                    move || {
                        flashcards.get(*current_index.get()).unwrap().flip();
                        confidence_rating.show();
                    }
                }) {{"flip"}}
                @ConfidenceRating: confidence_rating
            }
        }
    }
}
