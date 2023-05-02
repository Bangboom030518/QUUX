use crate::internal::prelude::*;

#[client]
pub trait Reactivity: Debug {
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>);
}

// #[client]
// impl<F> Reactivity for F
// where
//     F: FnMut(Rc<web_sys::Element>),
// {
//     fn apply(mut self: Box<Self>, element: Rc<web_sys::Element>) {
//         self(element);
//     }
// }

#[derive(Clone, Debug)]
pub struct Class {
    class: String,
    store: Store<bool>,
}

impl Class {
    #[must_use]
    pub fn new(class: &str, store: Store<bool>) -> Self {
        Self {
            class: class.to_string(),
            store,
        }
    }
}

#[client]
impl Reactivity for Class {
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>) {
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

#[client]
pub struct Many<'a, T, F, I>
where
    T: Clone,
    F: FnMut(T) -> Element<'a, I> + Clone,
    I: Item,
{
    list: store::List<T>,
    mapping: F,
}

#[client]
impl<'a, T, F, I> Debug for Many<'a, T, F, I>
where
    T: Clone,
    F: FnMut(T) -> Element<'a, I> + Clone,
    I: Item,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Many").finish()
    }
}

#[client]
impl<'a, T> Element<'a, T>
where
    T: Item,
{
    fn create_dom_element(&self) -> web_sys::Element {
        let dom_element = crate::dom::document()
            .create_element(&self.tag_name)
            .expect_internal("create element");
        for (key, value) in &self.attributes.attributes {
            dom_element
                .set_attribute(key, value)
                .expect_internal("add attribute");
        }
        dom_element.set_inner_html(&self.children.to_string());
        dom_element
    }
}

#[client]
impl<'a, T, F, I> Many<'a, T, F, I>
where
    T: Clone,
    F: FnMut(T) -> Element<'a, I> + Clone,
    I: Item,
{
    pub const fn new(list: store::List<T>, mapping: F) -> Self {
        Self { list, mapping }
    }
}

// TODO: reference cycle?

#[client]
impl<'a, T, F, I> Reactivity for Many<'a, T, F, I>
where
    T: Clone + 'a,
    F: FnMut(T) -> Element<'a, I> + 'static + Clone,
    I: Item + 'a,
{
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>) {
        use store::list::Event;

        let mut mapping = self.mapping.clone();
        self.list.on_change(move |event| match event {
            Event::Pop => element
                .last_element_child()
                .expect_internal("get last element of `ReactiveMany` list")
                .remove(),
            Event::Push(new) => {
                let mut new_element = mapping(new.clone());

                let dom_element = new_element.create_dom_element();
                element
                    .append_child(&dom_element)
                    .expect_internal("append child");
                new_element.dom_element = Some(Rc::new(dom_element));
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
