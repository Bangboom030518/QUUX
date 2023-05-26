use super::{flashcard, rating, Flashcard};
use crate::data::Term;
use quux::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Stack {
    terms: store::List<Term>,
    side: Store<flashcard::Side>,
}

impl component::Init for Stack {
    type Props = Vec<Term>;

    fn init(terms: Self::Props) -> Self {
        Self {
            terms: store::List::new(terms),
            side: Store::new(flashcard::Side::Term),
        }
    }
}

impl Component for Stack {
    fn render(self, _: Context<Self>) -> impl Item {
        let confidence_rating = rating::Selector::init(());
        let rating = confidence_rating.get_rating_store();

        div()
            .class("grid place-items-center gap-4")
            .child(
                div()
                    .class("flashcard-stack")
                    .reactive_many(self.terms.clone(), {
                        let side = self.side.clone();
                        move |_, term| {
                            let flashcard = Flashcard::new(term.clone(), side.clone());
                            div().component(flashcard)
                        }
                    }),
            )
            .child(
                button()
                    .class("btn")
                    .on(
                        "click",
                        event! {{
                            rating.on_change({
                                let confidence_rating = confidence_rating.clone();
                                let terms = self.terms.clone();
                                move |_, _| {
                                    terms.pop();
                                    confidence_rating.hide();
                                }
                            });

                            let side = self.side;

                            let confidence_rating = confidence_rating.clone();

                            move || {
                                let side_ref = *side.get();
                                let flipped = side_ref.flip();
                                side.set(flipped);
                                confidence_rating.show();
                            }
                        }},
                    )
                    .text("flip"),
            )
            .component(confidence_rating)
    }
}
