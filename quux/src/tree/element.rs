use super::{
    item::{RawHtml, Text},
    DisplayStore,
};
use crate::internal::prelude::*;
pub use reactivity::event;

pub mod html;
pub mod reactivity;

#[derive(Debug)]
pub struct Element<'a, T: Item> {
    tag_name: String,
    id: Option<u64>,
    attributes: Attributes,
    children: T,
    #[cfg(target_arch = "wasm32")]
    dom_element: Option<Rc<web_sys::Element>>,
    #[cfg(target_arch = "wasm32")]
    reactivity: Vec<Box<dyn reactivity::Reactivity + 'a>>,
    #[cfg(not(target_arch = "wasm32"))]
    _phantom: PhantomData<&'a ()>,
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
            #[cfg(not(target_arch = "wasm32"))]
            _phantom: PhantomData,
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

    #[client]
    fn hydrate(&mut self) {
        let dom_element = self.dom_element();

        for reactivity in &mut self.reactivity.drain(..) {
            reactivity.apply(Rc::clone(&dom_element));
        }

        self.children.hydrate();
    }

    #[client]
    fn dom_representation(&mut self) -> DomRepresentation {
        DomRepresentation::One(self.create_element(false).into())
    }
}

impl<'a, T: Item> Element<'a, T> {
    #[client]
    fn create_element(&mut self, hydrate: bool) -> web_sys::Element {
        let dom_element = crate::dom::document()
            .create_element(&self.tag_name)
            .expect_internal("create element");
        for (key, value) in &self.attributes.attributes {
            dom_element
                .set_attribute(key, value)
                .expect_internal("add attribute");
        }
        self.dom_element = Some(Rc::new(dom_element.clone()));
        for node in self.children.dom_representation() {
            dom_element
                .append_child(&node)
                .expect_internal("append node");
        }
        if hydrate {
            self.hydrate();
        }
        dom_element
    }

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
            #[cfg(not(target_arch = "wasm32"))]
            _phantom: PhantomData,
        }
    }

    pub fn text<S>(self, text: S) -> Element<'a, Pair<T, Text>>
    where
        S: Display,
    {
        self.child(Text::new(text))
    }

    pub fn raw_html<S>(self, html: S) -> Element<'a, Pair<T, RawHtml>>
    where
        S: Display,
    {
        self.child(RawHtml::new(html))
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
        F: FnMut() + 'static + Clone,
    {
        self.reactivity
            .push(Box::new(reactivity::Event::new(event, callback)));
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

    pub fn reactive_many<E, F, I>(mut self, list: store::List<E>, mut mapping: F) -> impl Item + 'a
    where
        E: Clone + 'a,
        I: Item + 'a,
        F: reactivity::many::Mapping<'a, E, I>,
        T: 'a,
    {
        let items = list.into_many(&mut mapping);

        #[cfg(target_arch = "wasm32")]
        {
            let many = reactivity::Many::new(list, mapping);
            self.reactivity.push(Box::new(many));
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
