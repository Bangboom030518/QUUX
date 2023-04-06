use crate::internal::prelude::*;

use super::BoxedComponents;

#[derive(Default, Clone)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Attributes,
    pub children: Children,
}

impl Display for Element {
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

impl Element {
    pub fn hydrate(&self) {
        self.components().hydrate();
    }

    pub fn components(&self) -> BoxedComponents {
        self.children.components()
    }

    pub fn new(tag_name: &str, attributes: Attributes, children: Children) -> Self {
        Self {
            tag_name: tag_name.to_string(),
            attributes,
            children,
            // components: Rc::new(components.clone()),
        }
    }
}
