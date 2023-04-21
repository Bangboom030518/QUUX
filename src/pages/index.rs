use super::Head;
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index;

impl Component for Index {
    fn render(self, context: Context<Self>) -> impl Item {
        type Component = Index;
        html()
            .attribute("lang", "en")
            .component(Head::new("QUUXLET - like Quizlet but gud"))
            .child(body().child(h1().text("Welcome to QUUXLET")).component(
                InitialisationScript::init(include_str!("../../dist/init.js")),
            ))

        // view! {
        //     context,
        //     html(lang="en", magic=true) {
        //         @Head("QUUXLET - like Quizlet but gud".to_string())
        //         body {
        //             h1 {{ "Welcome to QUUXLET" }}
        //             @InitialisationScript(include_str!("../../dist/init.js"))
        //         }
        //     }
        // }
    }
}
