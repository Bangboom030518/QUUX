use super::{DisplayStore, Hydrate};
use crate::internal::prelude::*;

#[derive(Default, Clone)]
pub struct Element<T: Children> {
    pub tag_name: String,
    pub attributes: Attributes,
    pub children: T,
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

impl<T: Children> Hydrate for Element<T> {
    fn hydrate(&self) {
        self.children.hydrate();
    }
}

impl Element<children::Empty> {
    #[must_use]
    pub fn new(tag_name: &str) -> Self {
        Self {
            tag_name: tag_name.to_string(),
            attributes: Attributes::default(),
            children: children::Empty,
        }
    }
}

impl<T: Children> Element<T> {
    #[must_use]
    pub fn attribute<V: ToString>(mut self, key: &str, value: V) -> Self {
        self.attributes
            .attributes
            .insert(key.to_string(), value.to_string());
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
            children: Pair(self.children, child),
        }
    }
}
