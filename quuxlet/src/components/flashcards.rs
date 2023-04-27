pub use confidence_rating::ConfidenceRating;
pub use flashcard::Flashcard;
use quux::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::{Ref},
};

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
}

impl component::Init for Flashcards {
    type Props = Vec<Term>;

    fn init(terms: Self::Props) -> Self {
        Self {
            terms: store::List::new(terms),
        }
    }
}

impl Component for Flashcards {
    fn render(self, _: Context<Self>) -> impl Item {
        // let confidence_rating: ConfidenceRating = todo!();
        // let flashcards: Rc<RefCell<Vec<Flashcard>>> = todo!();

        // TODO: for term in $self.terms { @Flashcard(term): flashcards }

        div()
            .class("grid place-items-center gap-4")
            .child(
                div().class("flashcard-stack").child(
                    Ref::<_>::from(&self.terms)
                        .iter()
                        .map(|term| Flashcard::init(term.clone()).render(Context::new()))
                        .collect::<Many<_>>(),
                ),
            )
            .child(
                button()
                    .class("btn")
                    .on(
                        "click",
                        event! {{
                            // let rating = confidence_rating.get_rating_store();
                            // let flashcards = Rc::clone(&flashcards);
                            // let terms = self.terms.clone();
                            // let confidence_rating = Rc::new(confidence_rating);

                            // rating.on_change({
                            //     let confidence_rating = Rc::clone(&confidence_rating);
                            //     move |_, _| {
                            //         terms.pop();
                            //         confidence_rating.hide();
                            //     }
                            // });

                            // move || {
                            //     let flashcards = flashcards.borrow();
                            //     let Some(flashcard) = flashcards.last() else {
                            //         quux::console_log!("No flashcards found");
                            //         return
                            //     };
                            //     flashcard.flip();
                            //     confidence_rating.show();
                            // }
                            || panic!("MUNNNNNNNEEEEEEEEEEEEE!")
                        }},
                    )
                    .text("flip"),
            )
            .component(ConfidenceRating::init(()))
    }
}
