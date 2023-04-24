use crate::internal::prelude::*;

pub struct Event {
    name: String,
    callback: Box<dyn FnMut() + 'static>,
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

    #[client]
    pub fn apply(self, element: &web_sys::Element) {
        use wasm_bindgen::prelude::*;

        let closure = Closure::wrap(self.callback as Box<dyn FnMut()>);

        element
            .add_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
            .expect_internal("add event");

        closure.forget();
    }
}

#[macro_export]
macro_rules! event {
    ($closure:expr) => {{
        #[cfg(target_arch = "wasm32")]
        {
            $closure
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            ()
        }
    }};
}

pub use event;
