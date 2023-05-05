#[client]
use super::Reactivity;
use crate::internal::prelude::*;
use std::fmt::Debug;

pub struct Event {
    name: String,
    callback: Box<dyn FnMut() + 'static>,
}

impl Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Event")
            .field("name", &self.name)
            .field("callback", &"unformatable!")
            .finish()
    }
}

impl Event {
    pub fn new<F>(name: &str, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        Self {
            name: name.to_string(),
            callback: Box::new(callback),
        }
    }
}

#[client]
impl Reactivity for Event {
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>) {
        use wasm_bindgen::prelude::*;

        let closure = Closure::wrap(self.callback);

        element
            .add_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
            .expect_internal("add event");

        closure.forget();
    }
}

#[macro_export]
macro_rules! callback {
    ($closure:expr) => {{
        #[cfg(target_arch = "wasm32")]
        {
            $closure
        }
        #[cfg(not(target_arch = "wasm32"))]
        {}
    }};
}

pub use callback as event;
