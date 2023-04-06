use super::BoxedComponents;
use crate::internal::prelude::*;

#[derive(Clone)]
pub enum Item {
    Element(Element),
    Expression(String),
}

impl Item {
    pub fn components(&self) -> BoxedComponents {
        match self {
            Self::Element(element) => element.components(),
            Self::Expression(_) => Box::new(()),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Element(element) => write!(f, "{element}"),
            Self::Expression(expression) => write!(f, "{expression}"),
        }
    }
}
