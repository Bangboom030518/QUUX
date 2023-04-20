use super::Head;
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index;

impl Component for Index {
    fn render(self, context: Context<Self>) -> Output<Self> {
        type Component = Index;
        view! {
            context,
            html(lang="en", magic=true) {
                @Head("QUUXLET - like Quizlet but gud".to_string())
                body {
                    h1 {{ "Welcome to QUUXLET" }}
                    @InitialisationScript(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}
