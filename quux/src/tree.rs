pub use element::Element;
pub use attributes::Attributes;
pub use item::Item;
pub use items::Items;
pub use components::Components;
pub use children::Children;
use crate::internal::prelude::*;

mod element;
mod attributes;
mod item;
mod items;
mod children;
mod components;

type DisplayStore = Store<Box<dyn Display>>;
type BoxedComponents = Box<dyn Components>;

pub mod prelude {
    pub use super::{Element, Attributes, Children, Item, Items, Components};
}
