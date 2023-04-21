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
    fn render(self, context: Context<Self>) -> impl Item
    where
        Self: Sized,
    {
        head()
            .child(meta().attribute("charset", "UTF-8"))
            .child(
                meta()
                    .attribute("http-equiv", "X-UA-Compatible")
                    .attribute("content", "IE=edge"),
            )
            .child(
                meta()
                    .attribute("name", "viewport")
                    .attribute("content", "width=device-width, initial-scale=1.0"),
            )
            .child(style().text(include_str!("../dist/output.css")))
            .child(self.title)
    }
}
