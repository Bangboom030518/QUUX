#[server]
use super::error::{self, Error};
use super::{nav_bar, Head};
use crate::Component;
pub use flashcard::Flashcard;
use quux::prelude::*;
pub use rating::Rating;
pub use stack::Stack;

pub mod flashcard;
pub mod rating;
pub mod stack;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set(super::super::Set);

#[server]
impl From<&sqlx::Error> for Error {
    fn from(value: &sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::SetNotFound,
            error => Self::Internal {
                message: error.to_string(),
            },
        }
    }
}

#[server]
impl Set {
    /// Fetches the set with `set_id` from the database
    /// # Errors
    /// If the database query fails
    pub async fn new(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        set_id: &str,
    ) -> Result<Self, error::Database> {
        Ok(Self::init(super::super::Set::fetch(pool, set_id).await?))
    }
}

impl quux::component::Init for Set {
    type Props = super::super::Set;

    fn init(set: Self::Props) -> Self {
        Self(set)
    }
}

impl Component for Set {
    fn render(self, _: Context<Self>) -> impl Item {
        html()
            .attribute("lang", "en")
            .component(Head::new(&format!("{} - QUUXLET", self.0.name)))
            .child(
                body()
                    .child(nav_bar())
                    .child(h1().text(self.0.name))
                    .component(Stack::init(self.0.terms))
                    .component(InitialisationScript::init(include_str!(
                        "../../dist/init.js"
                    ))),
            )
    }
}
