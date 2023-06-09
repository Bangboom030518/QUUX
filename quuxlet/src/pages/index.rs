use super::{nav_bar, Head};
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index;

impl From<()> for Index {
    fn from(value: ()) -> Self {
        Index
    }
}

impl Component for Index {
    fn render(self) -> impl Item {
        html()
            .attribute("lang", "en")
            .component(Head::new("QUUXLET - like Quizlet but gud"))
            .child(
                body()
                    .class("base-layout")
                    .child(nav_bar())
                    .child(main().child(h1().text("Welcome to QUUXLET")))
                    .component(InitialisationScript::new(include_str!(
                        "../../dist/init.js"
                    ))),
            )
    }
}
