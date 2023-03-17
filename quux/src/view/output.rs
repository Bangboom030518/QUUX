use super::SerializedComponent;
use crate::internal::prelude::*;

#[server]
pub struct Output<T>
where
    T: Component + Sized,
{
    pub html: String,
    pub component_node: SerializedComponent<T>,
}

#[server]
impl<T> Output<T>
where
    T: Component + Sized,
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
    T: Component + Sized,
{
    pub component: T,
}

#[client]
impl Output {
    pub fn new(component: T) {
        Self { component }
    }
}
