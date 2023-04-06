use crate::internal::prelude::*;

/// Everything the client needs to render a component.
///
/// These are created each time every time a component is used in a view, on the server.
#[derive(Serialize, Deserialize, Clone)]
pub struct SerializedComponent<T: Component> {
    pub component: T,
    pub render_context: super::Context<T>,
}

impl<T: Component> SerializedComponent<T> {
    pub fn render(self) -> Output<T> {
        self.component.render(self.render_context)
    }

    pub const fn new(component: T, render_context: super::Context<T>) -> Self {
        Self {
            component,
            render_context,
        }
    }
}
