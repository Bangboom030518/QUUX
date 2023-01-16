use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, Store};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
enum Rating {
    Terrible,
    Bad,
    Medium,
    Good,
    Perfect,
}

impl Default for Rating {
    fn default() -> Self {
        Self::Medium
    }
}

pub struct Props {
    pub is_visible: Store<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfidenceRating {
    is_visible: Store<bool>,
    rating: Store<Rating>,
}

impl Component for ConfidenceRating {
    type Props = Props;

    fn init(Props { is_visible }: Self::Props) -> Self {
        Self {
            is_visible,
            rating: Store::new(Rating::Medium),
        }
    }

    fn render(&self, context: shared::RenderContext) -> shared::RenderData {
        view! {
            div(class = "flashcard-hidden btn-group", class:active-when = (&self.is_visible, |visible: bool| !visible, "flashcard-hidden")) {
                button(class = "btn btn-error") {
                    {"Terrible"}
                }
                button(class = "btn btn-warning") {
                    {"Bad"}
                }
                button(class = "btn btn-primary") {
                    {"Medium"}   
                }
                button(class = "btn btn-secondary") {
                    {"Good"}
                }
                button(class = "btn btn-success") {
                    {"Perfect"}
                }
            }
        }
    }
}
