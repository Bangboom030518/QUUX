pub use create::Create;
pub use error::Error;
pub use index::Index;
pub use set::Set;

use quux::prelude::*;

mod create;
pub mod error;
mod index;
mod set;

#[derive(Serialize, Deserialize, Clone)]
pub struct Head {
    title: String,
}

impl Head {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}

impl component::Init for Head {
    type Props = String;

    fn init(props: Self::Props) -> Self {
        Self { title: props }
    }
}

impl Component for Head {
    fn render(self, context: Context<Self>) -> Output<Self>
    where
        Self: Sized,
    {
        type Component = Head;
        view! {
            context,
            head {
                meta(charset="UTF-8")
                meta("http-equiv"="X-UA-Compatible", content="IE=edge")
                meta(name="viewport", content="width=device-width, initial-scale=1.0")
                style {
                    { include_str!("../dist/output.css") }
                }
                title {{ self.title }}
            }
        }
    }
}
