pub use attributes::Attributes;
pub use element::Element;
pub use item::Item;
// pub use components::Components;
use crate::internal::prelude::*;
pub use children::Children;

mod attributes;
mod children;
mod element;
mod item;
// mod components;

pub trait ToString2 {
    fn to_string(&self) -> String;
}

impl<T: ToString> ToString2 for T {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

pub trait Hydrate {
    fn hydrate(&self) {}
}

type DisplayStore = Store<Box<dyn Display>>;

pub mod prelude {
    pub use super::{Attributes, Children, Element, Item};
}
