use crate::internal::prelude::*;

pub struct Server<T>
where
    T: Component,
{
    pub html: String,
    pub component_node: SerializedComponent<T>,
}

impl<T> Server<T>
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

#[derive(Default)]
pub struct Client<T>
where
    T: Component,
{
    pub component: T,
}

impl<T> Client<T>
where
    T: Component,
{
    pub const fn new(component: T) -> Self {
        Self { component }
    }
}

#[client]
pub type Output<T> = Client<T>;

#[server]
pub type Output<T> = Server<T>;
