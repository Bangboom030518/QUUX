use crate::internal::prelude::*;

#[server]
pub struct Output<T>
where
    T: Component,
{
    pub html: String,
    pub component_node: SerializedComponent<T>,
}

#[server]
impl<T> Output<T>
where
    T: Component,
{
    pub fn new(html: &str, component_node: SerializedComponent<T>) -> Self {
        Self {
            html: html.to_string(),
            component_node,
        }
    }
}

#[client]
#[derive(Default)]
pub struct Output<T>
where
    T: Component,
{
    pub component: T,
}

#[client]
impl<T> Output<T>
where
    T: Component,
{
    pub const fn new(component: T) -> Self {
        Self { component }
    }
}
