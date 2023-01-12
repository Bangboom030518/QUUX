use confidence_rating::ConfidenceRating;
use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, Store};

mod confidence_rating;

pub struct Props {
    pub term: &'static str,
    pub definition: &'static str,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Flashcard {
    term: String,
    definition: String,
    side: Store<Side>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    Term,
    Definition,
}

impl Side {
    #[cfg(target_arch = "wasm32")]
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

impl Flashcard {
    #[cfg(target_arch = "wasm32")]
    pub fn flip(&self) {
        let previous = *self.side.get();
        self.side.set(previous.flip());
    }
}

impl Component for Flashcard {
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
            div(class = "grid place-items-center gap-4 text-center") {
                div(class = "relative min-w-[60ch] min-h-[40ch]") {
                    div(
                        class = "card bg-base-200 shadow term absolute top-0 left-0 w-full h-full grid place-items-center transition-[opacity,transform] duration-300",
                        class:active-when = (&self.side, |side| side != Side::Term, "flashcard-hidden")
                    ) {
                        div(class = "card-body") {
                            p {{ self.term }}
                        }
                    }
                    div(
                        class = "card bg-base-200 shadow definition absolute top-0 left-0 w-full h-full grid place-items-center transition-[opacity,transform] duration-300 flashcard-hidden",
                        class:active-when = (&self.side, |side| side != Side::Definition, "flashcard-hidden")
                    ) {
                        div(class = "card-body") {
                            p {{ self.definition }}
                        }
                    }
                }
                button(class = "btn", on:click = {
                    let side = self.side.clone();
                    move || {
                        let previous = *side.get();
                        side.set(previous.flip());
                    }
                }) {{"flip"}}
                @ConfidenceRating
            }
        }
    }
}
