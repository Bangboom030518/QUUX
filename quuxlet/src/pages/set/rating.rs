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
pub struct Selector {
    is_hidden: Store<bool>,
    rating: Store<Rating>,
}

impl Selector {
    pub fn show(&self) {
        self.is_hidden.set(false);
    }

    pub fn hide(&self) {
        self.is_hidden.set(true);
    }

    pub fn get_rating_store(&self) -> Store<Rating> {
        self.rating.clone()
    }
}

impl Selector {
    pub fn new() -> Self {
        Self {
            is_hidden: Store::new(true),
            rating: Store::new(Rating::Ok),
        }
    }
}

// TODO: make `.text()` escape the basement (html)

fn rating_button(store: Store<Rating>, rating: Rating, svg: &str) -> impl Item {
    // tailwind include: btn-terrible btn-bad btn-ok btn-good btn-perfect

    button()
        .class(format!(
            "tooltip btn btn-icon btn-{}",
            rating.to_string().to_lowercase()
        ))
        .on("click", event!(move || store.set(rating)))
        .data_attribute("tip", rating)
        .attribute("title", rating)
        .raw_html(svg)
}

impl Component for Selector {
    fn render(self) -> impl Item {
        div()
            .class("btn-group flashcard-hidden")
            // TODO: remove need for duplication of reactive classes
            .reactive_class("flashcard-hidden", self.is_hidden)
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
    }
}
