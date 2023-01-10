use html::view;
use serde::{Deserialize, Serialize};
use shared::Component;

pub struct Props {
    pub term: &'static str,
    pub definition: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct Flashcard {
    term: String,
    definition: String,
}

impl Component for Flashcard {
    type Props = Props;

    fn init(props: Self::Props) -> Self {
        let Props { term, definition } = props;
        Self {
            term: term.to_string(),
            definition: definition.to_string(),
        }
    }

    fn render(&self, context: shared::RenderContext) -> shared::RenderData {
        view! {
            div {
                p {{ format!("The term is {}", self.term) }}
                p {{ format!("The definition is {}", self.definition) }}
            }
        }
    }
}
