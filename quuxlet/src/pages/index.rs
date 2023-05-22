use super::{nav_bar, Head};
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index;

impl Component for Index {
    fn render(self, _: Context<Self>) -> impl Item {
        html()
            .attribute("lang", "en")
            .component(Head::new("QUUXLET - like Quizlet but gud"))
            .child(
                body()
                    .child(nav_bar())
                    .child(h1().text("Welcome to QUUXLET"))
                    .component(InitialisationScript::init(include_str!(
                        "../../dist/init.js"
                    ))),
            )
    }
}
