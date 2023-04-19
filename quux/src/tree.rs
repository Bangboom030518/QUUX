pub use attributes::Attributes;
pub use element::Element;
pub use item::Item;
// pub use components::Components;
use crate::internal::prelude::*;
pub use children::Children;

mod attributes;
pub mod children;
mod component;
pub mod element;
mod item;

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
