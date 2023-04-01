pub use error::Error;
pub use set::Set;

use quux::prelude::*;

mod create;
mod error;
mod index;
mod set;

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
