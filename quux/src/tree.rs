pub use attributes::Attributes;
pub use element::Element;
pub use item::Item;
// pub use components::Components;
use crate::internal::prelude::*;
pub use children::Children;

mod attributes;
pub mod children;
pub mod element;
mod item;
mod component;

// TODO: gobble gobble gobble?
pub trait Hydrate {
    fn hydrate(&self) {}
}

type DisplayStore = Store<Box<dyn Display>>;

pub mod prelude {
    pub use super::{
        children::{self, Pair},
        component::ComponentNode,
        Attributes, Children, Element, Item,
    };
}
