use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, EmptyProps, Store};

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

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfidenceRating {
    rating: Store<Rating>,
}

impl Component for ConfidenceRating {
    type Props = ();

    fn init(_: Self::Props) -> Self {
        Self {
            rating: Store::new(Rating::Medium),
        }
    }

    fn render(&self, context: shared::RenderContext) -> shared::RenderData {
        view! {
            div {}
        }
    }
}
