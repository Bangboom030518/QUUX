#[cfg_server]
use super::error::{self, Error};
use super::{nav_bar, Head};
use crate::Component;
pub use flashcard::Flashcard;
use quux::{prelude::*, tree::element::html::html};
pub use rating::Rating;
pub use stack::Stack;

pub mod flashcard;
pub mod rating;
pub mod stack;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set(crate::data::Set);

#[cfg_server]
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

#[cfg_server]
impl Set {
    /// Fetches the set with `set_id` from the database
    /// # Errors
    /// If the database query fails
    pub async fn new(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        set_id: &str,
    ) -> Result<Self, error::Database> {
        Ok(Self(crate::data::Set::fetch(pool, set_id).await?))
    }
}

// impl Set {
//     fn new(set: crate::data::Set) -> Self {
//         Self(set)
//     }
// }

impl Component for Set {
    fn render(self) -> impl Item {
        html()
            .attribute("lang", "en")
            .component(Head::new(&format!("{} - QUUXLET", self.0.name)))
            .child(
                body()
                    .class("base-layout")
                    .child(nav_bar())
                    .child(
                        main()
                            .child(h1().text(self.0.name))
                            .component(Stack::new(self.0.terms)),
                    )
                    .component(InitialisationScript::new(include_str!(
                        "../../dist/init.js"
                    ))),
            )
    }
}
