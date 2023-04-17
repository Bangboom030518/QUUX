use crate::internal::prelude::*;

pub struct Output<T: Component> {
    pub element: crate::tree::Element<<T as ComponentChildren>::Children>,
    pub component: SerializedComponent<T>,
}

impl<T> Output<T>
where
    T: Component,
{
    pub const fn new(
        element: crate::tree::Element<<T as ComponentChildren>::Children>,
        component: SerializedComponent<T>,
    ) -> Self {
        Self { element, component }
    }
}
