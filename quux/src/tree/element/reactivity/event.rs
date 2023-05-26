#[client]
use super::Reactivity;
use crate::internal::prelude::*;
use std::fmt::Debug;

// TODO: closures leaked when event applied
// consider https://docs.rs/gloo-events/latest/gloo_events/

pub struct Event<F: FnMut() + 'static> {
    name: String,
    callback: F,
}

impl<F: FnMut() + 'static> Debug for Event<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Event")
            .field("name", &self.name)
            .field("callback", &"|event| { ... }")
            .finish()
    }
}

impl<F: FnMut() + 'static> Event<F> {
    pub fn new(name: &str, callback: F) -> Self {
        Self {
            name: name.to_string(),
            callback,
        }
    }
}

#[client]
impl<F: FnMut() + 'static> Reactivity for Event<F> {
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>) {
        use wasm_bindgen::prelude::*;

        let closure = Closure::wrap(Box::new(self.callback) as Box<dyn FnMut()>);

        element
            .add_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
            .expect_internal("add event");

        closure.forget();
    }
}
#[macro_export]
macro_rules! callback {
    ($closure:expr) => {
        #[allow(clippy::unit_arg)]
        {
            #[cfg(target_arch = "wasm32")]
            {
                $closure
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                ()
            }
        }
    };
}

pub use callback as event;
