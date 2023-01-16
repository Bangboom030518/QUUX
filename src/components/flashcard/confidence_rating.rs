use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, Store};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rating {
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
    is_visible: Store<bool>,
    rating: Store<Rating>,
}

impl ConfidenceRating {
    pub fn show(&self) {
        self.is_visible.set(true);
    }

    pub fn get_rating_store(&self) -> Store<Rating> {
        self.rating.clone()
    }
}

impl Component for ConfidenceRating {
    type Props = ();

    fn init(_: Self::Props) -> Self {
        Self {
            is_visible: Store::new(false),
            rating: Store::new(Rating::Medium),
        }
    }

    fn render(&self, context: shared::RenderContext) -> shared::RenderData {
        #[cfg(target_arch = "wasm32")]
        {
            shared::dom::console_log!("{}", *self.is_visible.get());
            self.is_visible
                .on_change(|_, new| shared::dom::console_log!("YAY2!!! {new}"))
        }

        view! {
            div(class = "flashcard-hidden btn-group", class:active-when = (&self.is_visible, |visible: bool| {
                shared::dom::console_log!("{visible}");
                !visible
            }, "flashcard-hidden")) {
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
