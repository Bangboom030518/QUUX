use std::fmt::Display;

use quux::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rating {
    Terrible,
    Bad,
    Ok,
    Good,
    Perfect,
}

impl Default for Rating {
    fn default() -> Self {
        Self::Ok
    }
}

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConfidenceRating {
    is_visible: Store<bool>,
    rating: Store<Rating>,
}

impl ConfidenceRating {
    pub fn show(&self) {
        self.is_visible.set(true);
    }

    pub fn hide(&self) {
        self.is_visible.set(false);
    }

    pub fn get_rating_store(&self) -> Store<Rating> {
        self.rating.clone()
    }
}

impl component::Init for ConfidenceRating {
    type Props = ();

    fn init(_: Self::Props) -> Self {
        Self {
            is_visible: Store::new(false),
            rating: Store::new(Rating::Ok),
        }
    }
}

// TODO: make `.text()` escape the basement

fn rating_button(store: Store<Rating>, rating: Rating, svg: &str) -> impl Item {
    button()
        .class(format!(
            "tooltip btn btn-icon btn-{}",
            rating.to_string().to_lowercase()
        ))
        .on("click", event!(move || store.set(rating)))
        .data_attribute("tip", rating)
        .attribute("title", rating)
        .text(svg)
}

impl Component for ConfidenceRating {
    fn render(self, _: Context<Self>) -> impl Item {
        div()
            .class("flashcard-hidden btn-group")
            .child(rating_button(
                self.rating.clone(),
                Rating::Terrible,
                include_str!("../../../assets/terrible.svg"),
            ))
            .child(rating_button(
                self.rating.clone(),
                Rating::Bad,
                include_str!("../../../assets/bad.svg"),
            ))
            .child(rating_button(
                self.rating.clone(),
                Rating::Ok,
                include_str!("../../../assets/ok.svg"),
            ))
            .child(rating_button(
                self.rating.clone(),
                Rating::Good,
                include_str!("../../../assets/good.svg"),
            ))
            .child(rating_button(
                self.rating,
                Rating::Perfect,
                include_str!("../../../assets/perfect.svg"),
            ))

        // TODO: class:active-when" = (&self.is_visible, |visible: bool| !visible, "flashcard-hidden")
    }
}
