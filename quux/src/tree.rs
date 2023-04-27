pub use attributes::Attributes;
pub use element::Element;
pub use item::Item;
// pub use components::Components;
use crate::internal::prelude::*;

mod attributes;
mod component;
pub mod element;
pub mod event;
pub mod item;

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
        event,
        item::{self, Many, Pair},
        Attributes, Element, Item,
    };
}
