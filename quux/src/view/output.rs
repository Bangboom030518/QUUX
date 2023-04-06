use crate::internal::prelude::*;

pub struct Output<T> where T: Component {
    pub element: crate::tree::Element,
    pub component: SerializedComponent<T>,
}

impl<T> Output<T> where T: Component {
    pub const fn new(element: crate::tree::Element, component: SerializedComponent<T>) -> Self {
        Self { element, component }
    }
}