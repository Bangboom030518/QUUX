use crate::internal::prelude::*;
pub use class::Class;
pub use event::{event, Event};
#[client]
pub use many::Many;

mod class;
mod event;
#[cfg(target_arch = "wasm32")]
mod many;

#[client]
pub trait Reactivity: Debug {
    fn apply(&mut self, element: Rc<web_sys::Element>);
}
