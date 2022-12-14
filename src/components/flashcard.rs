use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, Store};

pub struct Props {
    pub term: &'static str,
    pub definition: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct Flashcard<'a> {
    term: String,
    definition: String,
    side: Store<'a, Side>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Term,
    Definition,
}

impl Side {
    fn flip(self) -> Self {
        match self {
            Self::Term => Self::Definition,
            Self::Definition => Self::Term,
        }
    }
}

impl Default for Side {
    fn default() -> Self {
        Self::Term
    }
}

impl<'a> Flashcard<'a> {
    pub fn flip(&self) {
        let previous = *self.side.get();
        self.side.set(previous.flip());
    }
}

impl<'a> Component for Flashcard<'a> {
    type Props = Props;

    fn init(props: Self::Props) -> Self {
        let Props { term, definition } = props;
        Self {
            term: term.to_string(),
            definition: definition.to_string(),
            side: Store::new(Side::Term),
        }
    }

    fn render(&self, context: shared::RenderContext) -> shared::RenderData {
        view! {
            div(class = "relative w-[50ch] h-[20ch]") {
                div(class = "card bg-base-200 shadow term absolute top-0 left-0 w-full h-full", class:active-when = (&self.side, |side| side == Side::Term, "hidden")) {
                    div(class = "card-body") {
                        p {{ self.term }}
                    }
                }
                div(class = "card bg-base-200 shadow definition absolute top-0 left-0 w-full h-full") {
                    div(class = "card-body") {
                        p {{ self.definition }}
                    }
                }
            }
        }
    }
}
