#[client]
use super::Reactivity;
use crate::internal::prelude::*;
use std::fmt::Debug;

// TODO: `'static + Clone`?
pub struct Event<F: FnMut() + 'static + Clone> {
    name: String,
    callback: F,
}

impl<F: FnMut() + 'static + Clone> Debug for Event<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Event")
            .field("name", &self.name)
            .field("callback", &"unformatable!")
            .finish()
    }
}

impl<F: FnMut() + 'static + Clone> Event<F> {
    pub fn new(name: &str, callback: F) -> Self {
        Self {
            name: name.to_string(),
            callback,
        }
    }
}

#[client]
impl<F: FnMut() + 'static + Clone> Reactivity for Event<F> {
    fn apply(&mut self, element: Rc<web_sys::Element>) {
        use wasm_bindgen::prelude::*;

        let closure = Closure::wrap(Box::new(self.callback.clone()) as Box<dyn FnMut()>);

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
