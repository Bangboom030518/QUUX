use crate::internal::prelude::*;
pub use class::Class;
pub use event::{event, Event};
#[client]
pub use many::Many;

mod class;
mod event;
pub mod many;

#[client]
pub trait Reactivity: Debug {
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>);
}
