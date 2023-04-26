#[server]
use super::error::{self, Error};
use super::Head;
use crate::{components::Flashcards, Component};
use quux::prelude::*;

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
            .component(Head::new("Flashcards - QUUX"))
            .child(
                body()
                    .child(h1().text("Welcome to QUUXLET"))
                    .component(Flashcards::init(self.0.terms))
                    .component(InitialisationScript::init(include_str!(
                        "../../dist/init.js"
                    ))),
            )
    }
}
