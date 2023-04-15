use super::Hydrate;
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
            self.tag_name,
            self.attributes,
            self.children.to_string()
        )
    }
}

impl<T: Children> Hydrate for Element<T> {
    fn hydrate(&self) {
        self.children.hydrate();
    }
}

impl<T: Children> Element<T> {
    pub fn new(tag_name: &str, attributes: Attributes, children: T) -> Self {
        Self {
            tag_name: tag_name.to_string(),
            attributes,
            children,
        }
    }
}
