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
pub mod event;
mod item;

// TODO: gobble gobble gobble?
pub trait Hydrate {
    fn hydrate(self)
    where
        Self: Sized,
    {
    }
}

type DisplayStore = Store<Box<dyn Display>>;

pub mod prelude {
    pub use super::{
        children::{self, Pair},
        component::ComponentNode,
        event, Attributes, Children, Element, Item,
    };
}
