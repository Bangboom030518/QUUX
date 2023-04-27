use super::DisplayStore;
use crate::internal::prelude::*;
pub use reactivity::event;
use reactivity::Event;

pub mod html;
pub mod reactivity;

pub struct Element<'a, T: Item> {
    tag_name: String,
    id: Option<u64>,
    attributes: Attributes,
    children: T,
    #[cfg(target_arch = "wasm32")]
    dom_element: Option<Rc<web_sys::Element>>,
    #[cfg(target_arch = "wasm32")]
    reactivity: Vec<Box<dyn reactivity::Reactivity + 'a>>,
}

impl<'a, T: Item> Display for Element<'a, T> {
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
impl<'a, T: Item> super::Hydrate for Element<'a, T> {
    #[client]
    fn hydrate(mut self) {
        let dom_element = self.dom_element();

        for reactivity in self.reactivity {
            reactivity.apply(Rc::clone(&dom_element));
        }

        // for event in self.events {
        //     event.apply(Rc::clone(&dom_element));
        // }

        self.children.hydrate();
    }
}

impl<'a> Element<'a, item::Empty> {
    #[must_use]
    pub fn new(tag_name: &str) -> Self {
        Self {
            tag_name: tag_name.to_string(),
            attributes: Attributes::default(),
            children: item::Empty,
            id: None,
            #[cfg(target_arch = "wasm32")]
            dom_element: None,
            #[cfg(target_arch = "wasm32")]
            reactivity: Vec::new(),
        }
    }
}

impl<'a, T: Item> Item for Element<'a, T> {
    fn insert_id(&mut self, id: u64) -> u64 {
        self.id = Some(id);
        self.attributes
            .attributes
            .insert("data-quux-id".to_string(), id.to_string());
        self.children.insert_id(id + 1)
    }
}

impl<'a, T: Item> Element<'a, T> {
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
    pub fn child<I: Item>(self, child: I) -> Element<'a, Pair<T, I>> {
        Element {
            tag_name: self.tag_name,
            attributes: self.attributes,
            id: self.id,
            children: Pair(self.children, child),
            #[cfg(target_arch = "wasm32")]
            reactivity: self.reactivity,
            #[cfg(target_arch = "wasm32")]
            dom_element: self.dom_element,
        }
    }

    pub fn text<S>(self, text: S) -> Element<'a, Pair<T, String>>
    where
        S: Display,
    {
        self.child(text.to_string())
    }

    pub fn component<C>(self, component: C) -> Element<'a, Pair<T, impl Item>>
    where
        C: Component + Clone,
    {
        self.child(component.render(Context::new()))
    }

    #[server]
    #[must_use]
    pub const fn on(self, _: &str, _: ()) -> Self {
        self
    }

    #[must_use]
    #[client]
    pub fn on<F>(mut self, event: &str, callback: F) -> Self
    where
        F: FnMut() + 'static,
    {
        self.reactivity.push(Box::new(Event::new(event, callback)));
        self
    }

    #[must_use]
    #[server]
    pub fn reactive_class(self, _: &str, _: Store<bool>) -> Self {
        self
    }

    #[client]
    #[must_use]
    /// # Panics
    /// if it fails to toggle the class in the dom
    pub fn reactive_class(mut self, class: &str, store: Store<bool>) -> Self {
        self.reactivity
            .push(Box::new(reactivity::Class::new(class, store)));
        self
    }

    pub fn reactive_many<E, F, I>(mut self, list: store::List<E>, mut mapping: F) -> impl Item
    where
        E: Clone + 'a,
        I: Item + 'a,
        F: FnMut(&E) -> Element<I> + 'a,
    {
        // TODO: what??
        let items = list
            .clone()
            .into_iter()
            .collect::<Vec<_>>()
            .iter()
            .map(&mut mapping)
            .collect::<Many<_>>();

        #[cfg(target_arch = "wasm32")]
        {
            let parent = self.dom_element();
            let many = reactivity::Many::new(parent, list, mapping);
            self.reactivity.push(Box::new(many))
        }

        self.child(items)
    }
    #[client]
    pub fn dom_element(&mut self) -> Rc<web_sys::Element> {
        Rc::clone(self.dom_element.get_or_insert_with(|| {
            Rc::new(crate::dom::get_reactive_element(
                self.id.expect_internal("get reactive element"),
            ))
        }))
    }
}
