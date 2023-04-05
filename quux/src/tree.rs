use crate::internal::prelude::*;
use std::collections::HashMap;

type DisplayStore = Store<Box<dyn Display>>;

#[derive(Clone, Default)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Attributes,
    pub children: Children,
}

impl std::fmt::Display for Element {
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

#[derive(Clone, Default)]
pub struct Items(Vec<Item>);

#[derive(Clone)]
pub enum Children {
    Items(Items),
    ReactiveStore(DisplayStore),
    SelfClosing, // ForLoop(ForLoop),
}

impl Children {
    pub fn is_self_closing(&self) -> bool {
        matches!(self, Self::SelfClosing)
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::Items(Items::default())
    }
}

#[derive(Clone)]
pub enum Item {
    Element(Element),
    Expression(String),
}

#[derive(Default, Clone)]
pub struct Attributes {
    pub attributes: HashMap<String, String>,
    pub reactive_attributes: HashMap<String, DisplayStore>,
}

fn render() -> Element {
    /*
    <div>Hello</div>
    */
    Element {
        tag_name: "div".to_string(),
        attributes: Attributes::default(),
        children: Children::Items(Items(vec![Item::Expression("Hello".to_string())])),
    }
}
