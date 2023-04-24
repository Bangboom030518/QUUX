use super::DisplayStore;
use crate::internal::prelude::*;
use event::Event;

pub mod html;

#[derive(Default)]
pub struct Element<T: Children> {
    tag_name: String,
    id: u64,
    attributes: Attributes,
    children: T,
    #[cfg(target_arch = "wasm32")]
    dom_element: Option<web_sys::Element>,
    #[cfg(target_arch = "wasm32")]
    events: Vec<Event>,
}

impl<T: Children> Display for Element<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.children.is_self_closing() {
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
impl<T: Children> super::Hydrate for Element<T> {
    #[client]
    fn hydrate(mut self) {
        let dom_element = self.dom_element().clone();

        for event in self.events {
            event.apply(&dom_element);
        }

        self.children.hydrate();
    }
}

#[client]
impl<T: Children> Element<T> {
    #[allow(clippy::missing_panics_doc)]
    pub fn dom_element(&mut self) -> &web_sys::Element {
        self.dom_element
            .get_or_insert_with(|| crate::dom::get_reactive_element(self.id));

        self.dom_element.as_ref().unwrap()
    }
}

impl Element<children::Empty> {
    pub fn new(tag_name: &str) -> Self {
        use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

        static ID: AtomicU64 = AtomicU64::new(0);

        Self {
            tag_name: tag_name.to_string(),
            attributes: Attributes::default(),
            children: children::Empty,
            id: ID.fetch_add(1, Relaxed),
            #[cfg(target_arch = "wasm32")]
            dom_element: None,
            #[cfg(target_arch = "wasm32")]
            events: Vec::new(),
        }
    }
}

impl<T: Children> Element<T> {
    #[must_use]
    pub fn attribute<V: Display>(mut self, key: &str, value: V) -> Self {
        self.attributes
            .attributes
            .insert(key.to_string(), value.to_string());
        self
    }

    #[must_use]
    pub fn id<V: Display>(self, value: V) -> Self {
        self.attribute("id", value)
    }

    #[must_use]
    pub fn class<V: Display>(self, value: V) -> Self {
        self.attribute("class", value)
    }

    #[must_use]
    pub fn data_attribute<V: Display>(self, key: &str, value: V) -> Self {
        self.attribute(&format!("data-{key}"), value)
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
            #[cfg(target_arch = "wasm32")]
            dom_element: self.dom_element,
        }
    }

    pub fn text<S>(self, text: S) -> Element<Pair<T, String>>
    where
        S: Display,
    {
        self.child(text.to_string())
    }

    pub fn component<C>(self, component: C) -> Element<Pair<T, ComponentNode<C>>>
    where
        C: Component + Clone,
    {
        self.child(ComponentNode(component))
    }

    #[server]
    pub fn on(self, _: &str, _: ()) -> Self {
        self
    }

    #[must_use]
    #[client]
    pub fn on<F>(mut self, event: &str, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.events.push(Event::new(event, callback));
        self
    }
}
