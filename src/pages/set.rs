#[server]
use super::error::Error;
use super::Head;
use crate::{components::Flashcards, Component};
use quux::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set(super::super::Set);

#[server]
impl Set {
    pub async fn new(pool: &sqlx::Pool<sqlx::Sqlite>, set_id: &str) -> Result<Self, Error> {
        match super::super::Set::fetch(pool, set_id).await {
            Ok(set) => Ok(Self::init(set)),
            Err(error) => Err(match error {
                sqlx::Error::RowNotFound => todo!(), // (StatusCode::NOT_FOUND, "Set not found :(".to_string()),
                _ => Error::init(Box::new(error)),
            }),
        }
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
