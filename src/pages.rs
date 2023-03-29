// pub use index::Index;
// pub use not_found::NotFound;
pub use error::ServerError;
pub use set::Set;
mod error;
mod index;
mod not_found;
mod set;

use quux::prelude::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Head {
    title: String,
}

impl component::Init for Head {
    type Props = String;

    fn init(props: Self::Props) -> Self {
        Self { title: props }
    }
}

impl Component for Head {
    fn render(self, context: quux::view::Context<Self>) -> quux::view::Output<Self>
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
