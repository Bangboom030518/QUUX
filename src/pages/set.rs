use super::server_error::ServerError;
use crate::{components::Flashcards, Component};
use quux::prelude::*;
// note: required for `quux::render::Output<set::Set>` to implement `Into<EnumRenderOutput<set::ComponentEnum>>`

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

impl<T> Component<T> for Set {
    type Props = super::super::Set;
    type ComponentEnum = crate::ComponentEnum;

    fn init(set: super::super::Set) -> Self {
        Self(set)
    }

    fn render(self, context: render::Context<Self::ComponentEnum>) -> render::Output<Self> {
        view! {
            context, T,
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
                    @InitialisationScript<Self::ComponentEnum>(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl axum::response::IntoResponse for Set {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html::from(self.render_to_string()).into_response()
    }
}
