use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

use super::{DisplayStore, Hydrate};
use crate::internal::prelude::*;

pub mod html;

#[derive(Clone)]
pub struct Event {
    name: String,
    callback: js_sys::Function,
}

impl Event {
    pub fn new<F>(name: &str, callback: F) -> Self
    where
        F: FnMut(),
    {
        use wasm_bindgen::prelude::*;

        let closure = Closure::wrap(Box::new(callback) as Box<dyn FnMut()>);
        let callback = *closure.as_ref().unchecked_ref();
        closure.forget();
        Self {
            name: name.to_string(),
            callback,
        }
    }
}

// TODO: consider caching web sys elements in this struct

#[derive(Default, Clone)]
pub struct Element<T: Children> {
    tag_name: String,
    id: u64,
    attributes: Attributes,
    children: T,
    #[cfg(target_arch = "wasm32")]
    events: Vec<Event>,
}

impl<T: Children> Display for Element<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if T::SELF_CLOSING {
            return write!(f, "<{} {} />", self.tag_name, self.attributes);
        }
        write!(
            f,
            "<{0} {1}>{2}</{0}>",
            self.tag_name, self.attributes, self.children
        )
    }
}
// TODO: move `Hydrate` trait to client only

impl<T: Children> Hydrate for Element<T> {
    #[client]
    fn hydrate(&self) {
        for event in self.events {
            crate::dom::get_reactive_element(self.id)
                .add_event_listener_with_callback(&event.name, &event.callback)
                .expect_internal("add event");
        }

        self.children.hydrate();
    }
}

impl Element<children::Empty> {
    #[must_use]
    pub fn new(tag_name: &str) -> Self {
        static ID: AtomicU64 = AtomicU64::new(0);

        Self {
            tag_name: tag_name.to_string(),
            attributes: Attributes::default(),
            children: children::Empty,
            id: ID.fetch_add(1, Relaxed),
            #[cfg(target_arch = "wasm32")]
            events: Vec::new(),
        }
    }
}

#[macro_export]
macro_rules! event_function {
    ($closure:expr) => {
        quux::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                $closure
            } else {
                ()
            }
        }
    };
}

pub use event_function;

impl<T: Children> Element<T> {
    #[must_use]
    pub fn attribute<V: Display>(mut self, key: &str, value: V) -> Self {
        self.attributes
            .attributes
            .insert(key.to_string(), value.to_string());
        self
    }

    #[must_use]
    pub fn id<V: Display>(mut self, value: V) -> Self {
        self.attribute("id", value);
        self
    }

    #[must_use]
    pub fn class<V: Display>(mut self, value: V) -> Self {
        self.attribute("class", value);
        self
    }

    #[must_use]
    pub fn data_attribute<V: Display>(mut self, key: &str, value: V) -> Self {
        self.attribute(&format!("data-{key}"), value);
        self
    }

    #[must_use]
    pub fn reactive_attribute(mut self, key: &str, value: DisplayStore) -> Self {
        self.attributes
            .reactive_attributes
            .insert(key.to_string(), value);
        self
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn child<I: Item>(self, child: I) -> Element<Pair<T, I>> {
        Element {
            tag_name: self.tag_name,
            attributes: self.attributes,
            id: self.id,
            children: Pair(self.children, child),
            #[cfg(target_arch = "wasm32")]
            events: self.events,
        }
    }

    #[server]
    pub fn on<F>(self, event: &str) -> Self
    where
        F: FnMut(),
    {
        self
    }

    #[client]
    pub fn on<F>(mut self, event: &str, callback: F) -> Self
    where
        F: FnMut(),
    {
        self.events.push(Event::new(event, callback));
        self
    }
}
