use super::server_error::ServerError;
use crate::{components::Flashcards, Component};
use quux::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set(super::super::Set);

#[cfg(not(target_arch = "wasm32"))]
impl Set {
    pub async fn new(pool: &sqlx::Pool<sqlx::Sqlite>, set_id: &str) -> Result<Self, ServerError> {
        match super::super::Set::fetch(pool, set_id).await {
            Ok(set) => Ok(Self::init(set)),
            Err(error) => Err(match error {
                sqlx::Error::RowNotFound => todo!(), // (StatusCode::NOT_FOUND, "Set not found :(".to_string()),
                _ => ServerError::init(Box::new(error)),
            }),
        }
    }
}

impl quux::component::Init for Set {
    type Props = super::super::Set;

    fn init(set: super::super::Set) -> Self {
        Self(set)
    }
}

impl Component for Set {
    fn render(self, context: Context<Self>) -> Output<Self> {
        type Component = Set;
        view! {
            context,
            html(lang="en") {
                head {
                    meta(charset="UTF-8") {}
                    meta("http-equiv"="X-UA-Compatible", content="IE=edge") {}
                    meta(name="viewport", content="width=device-width, initial-scale=1.0") {}
                    style {
                        { include_str!("../../dist/output.css") }
                    }
                    title {{ "Document" }}
                }
                body {
                    h1 {{ "Welcome to Quuxlet" }}
                    @Flashcards(self.0.terms.clone())
                    @InitialisationScript(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}