use crate::QUUXComponentEnum;
use quux::prelude::*;
use quux::{Component, ComponentEnum, Store};
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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
    type ComponentEnum = QUUXComponentEnum;

    fn init(_: Self::Props) -> Self {
        Self {
            is_visible: Store::new(false),
            rating: Store::new(Rating::Ok),
        }
    }

    fn render(
        &self,
        context: quux::RenderContext<Self::ComponentEnum>,
    ) -> quux::RenderData<Self::ComponentEnum> {
        
        view! {
            div(class = "flashcard-hidden btn-group", class:active-when = (&self.is_visible, |visible: bool| !visible, "flashcard-hidden")) {
                button(class = "tooltip btn btn-icon btn-terrible", on:click = {
                    let rating = self.rating.clone();
                    move || rating.set(Rating::Terrible)
                }, data-tip="Terrible", title="Terrible") {
                    {include_str!("../../../assets/terrible.svg")}
                }
                button(class = "tooltip btn btn-icon btn-bad", on:click = {
                    let rating = self.rating.clone();
                    move || rating.set(Rating::Bad)
                }, data-tip = "Bad", title = "Bad") {
                    {include_str!("../../../assets/bad.svg")}
                }
                button(class = "tooltip btn btn-icon btn-ok", on:click = {
                    let rating = self.rating.clone();
                    move || rating.set(Rating::Ok)
                }, data-tip = "Ok", title = "Ok") {
                    {include_str!("../../../assets/ok.svg")}
                }
                button(class = "tooltip btn btn-icon btn-good", on:click = {
                    let rating = self.rating.clone();
                    move || rating.set(Rating::Good)
                }, data-tip = "Good", title = "Good") {
                    {include_str!("../../../assets/good.svg")}
                }
                button(class = "tooltip btn btn-icon btn-perfect", on:click = {
                    let rating = self.rating.clone();
                    move || rating.set(Rating::Perfect)
                }, data-tip = "Perfect", title = "Perfect") {
                    {include_str!("../../../assets/perfect.svg")}
                }
            }
        }
    }
}
