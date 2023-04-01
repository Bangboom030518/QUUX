#[server]
use super::error::Error;
use super::Head;
use crate::{components::Flashcards, Component};
use quux::prelude::*;

#[derive(thiserror::Error, Debug)]
#[error("set not found")]
pub struct NotFound;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set(super::super::Set);

#[server]
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Error::SetNotFound,
            error => Error::Internal {
                message: error.to_string(),
            },
        }
    }
}

#[server]
impl Set {
    pub async fn new(pool: &sqlx::Pool<sqlx::Sqlite>, set_id: &str) -> Result<Self, Error> {
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
    fn render(self, context: Context<Self>) -> Output<Self> {
        type Component = Set;
        view! {
            context,
            html(lang="en") {
                @Head("Flashcards - QUUX".to_string())
                body {
                    h1 {{ "Welcome to Quuxlet" }}
                    @Flashcards(self.0.terms.clone())
                    @InitialisationScript(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}
