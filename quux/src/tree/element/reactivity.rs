use crate::internal::prelude::*;

#[client]
pub trait Reactivity {
    fn apply(self, element: Rc<web_sys::Element>);
}

#[client]
impl<F> Reactivity for F
where
    F: FnMut(Rc<web_sys::Element>),
{
    fn apply(mut self, element: Rc<web_sys::Element>) {
        self(element)
    }
}

#[derive(Clone)]
pub struct Class {
    class: String,
    store: Store<bool>,
}

impl Class {
    pub fn new(class: &str, store: Store<bool>) -> Self {
        Self {
            class: class.to_string(),
            store,
        }
    }
}

#[client]
impl Reactivity for Class {
    fn apply(self, element: Rc<web_sys::Element>) {
        let class = self.class.to_string();
        // TODO: simpler method?
        self.store.on_change(move |_, &enabled| {
            let class_list = element.class_list();
            if enabled {
                class_list.add_1(&class).unwrap();
            } else {
                class_list.remove_1(&class).unwrap();
            }
        });
    }
}

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
}

#[client]
impl Reactivity for Event {
    fn apply(self, element: Rc<web_sys::Element>) {
        use wasm_bindgen::prelude::*;

        let closure = Closure::wrap(self.callback);

        element
            .add_event_listener_with_callback(&self.name, closure.as_ref().unchecked_ref())
            .expect_internal("add event");

        closure.forget();
    }
}

pub struct Many<T, F, I>
where
    T: Clone,
    F: FnMut(&T) -> Element<I>,
    I: Item,
{
    parent: Rc<web_sys::Element>,
    list: store::List<T>,
    mapping: F,
}
#[client]
impl<T> Element<T>
where
    T: Item,
{
    fn create_dom_element(&self) -> web_sys::Element {
        let dom_element = crate::dom::document()
            .create_element(&self.tag_name)
            .expect_internal("create element");
        for (key, value) in self.attributes.attributes {
            dom_element
                .set_attribute(&key, &value)
                .expect_internal("add attribute")
        }
        dom_element.set_inner_html(&self.children.to_string());
        dom_element
    }
}

#[client]
impl<T, F, I> Many<T, F, I>
where
    T: Clone,
    F: FnMut(&T) -> Element<I>,
    I: Item,
{
    pub fn new(parent: Rc<web_sys::Element>, list: store::List<T>, mapping: F) -> Self {
        Self {
            parent,
            list,
            mapping,
        }
    }
}

#[client]
impl<T, F, I> Reactivity for Many<T, F, I>
where
    T: Clone,
    F: FnMut(&T) -> Element<I>,
    I: Item,
{
    fn apply(mut self, element: Rc<web_sys::Element>) {
        use store::list::Event;

        self.list.on_change(|event| match event {
            Event::Pop(value, index) => element
                .last_element_child()
                .expect_internal("get last element of `ReactiveMany` list")
                .remove(),
            Event::Push(new) => {
                let mut element = (self.mapping)(new);

                let dom_element = element.create_dom_element();
                element.dom_element = Some(Rc::new(dom_element));
                self.parent.append_child(&dom_element);
            }
        });
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
        {}
    }};
}

pub use event;
