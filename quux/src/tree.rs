pub use attributes::Attributes;
pub use element::Element;
pub use item::Item;
// pub use components::Components;
use crate::internal::prelude::*;
pub use children::Children;

mod attributes;
pub mod children;
mod element;
mod item;
// mod components;

pub trait Hydrate {
    fn hydrate(&self) {}
}

type DisplayStore = Store<Box<dyn Display>>;

pub mod prelude {
    pub use super::{
        children::{self, Pair},
        Attributes, Children, Element, Item,
    };
}
