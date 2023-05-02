pub use confidence_rating::ConfidenceRating;
pub use flashcard::Flashcard;
use quux::prelude::*;
use serde::{Deserialize, Serialize};
use std::cell::Ref;

pub mod confidence_rating;
pub mod flashcard;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Set {
    pub terms: Vec<Term>,
    pub name: String,
}

impl Set {
    #[server]
    pub async fn fetch(pool: &sqlx::Pool<sqlx::Sqlite>, set_id: &str) -> Result<Self, sqlx::Error> {
        use sqlx::query::Map;

        let query: Map<_, _, _> = sqlx::query!("SELECT sets.name FROM sets WHERE id = ?", set_id);
        let name = query.fetch_one(pool).await?.name;

        let query: Map<_, _, _> = sqlx::query!(
            "SELECT terms.term, terms.definition FROM terms WHERE set_id = ?",
            set_id
        );

        let terms = query
            .fetch_all(pool)
            .await?
            .into_iter()
            .map(|row| Term::new(&row.term, &row.definition))
            .collect();

        Ok(Self { terms, name })
    }
}

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
pub struct Flashcards {
    terms: store::List<Term>,
    side: Store<flashcard::Side>,
}

impl component::Init for Flashcards {
    type Props = Vec<Term>;

    fn init(terms: Self::Props) -> Self {
        Self {
            terms: store::List::new(terms),
            side: Store::new(flashcard::Side::Term),
        }
    }
}

struct ForLoop<T, I: Item> {
    list: store::List<T>,
    mapping: Box<dyn FnMut(&T) -> I>,
}

impl<T, I: Item> Component for ForLoop<T, I> {
    fn render(self, _: quux::context::Context<Self>) -> impl Item
    where
        Self: Sized,
    {
        div().child(
            Ref::<_>::from(&self.list)
                .iter()
                .map(self.mapping)
                .collect::<Many<_>>(),
        )
    }
}

// struct ForLoop<T, C>
// where
//     C: Component, {}

impl Component for Flashcards {
    fn render(self, _: Context<Self>) -> impl Item {
        let confidence_rating = ConfidenceRating::init(());
        let rating = confidence_rating.get_rating_store();

        div()
            .class("grid place-items-center gap-4")
            .child(
                div()
                    .class("flashcard-stack")
                    .reactive_many(self.terms.clone(), {
                        let side = self.side.clone();
                        move |term| {
                            let flashcard = Flashcard::new(term, side.clone());
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
