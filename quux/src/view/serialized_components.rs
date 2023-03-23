use crate::internal::prelude::*;

/// Everything the client needs to render a component.
///
/// These are created each time every time a component is used in a view, on the server.
#[derive(Serialize, Deserialize, Clone)]
pub struct SerializedComponent<T: Component> {
    pub component: T,
    pub render_context: <T as super::ClientContext>::Context,
}

impl<T: Component> SerializedComponent<T> {
    #[client]
    pub fn render(self) -> Output<T> {
        self.component.render(self.render_context)
    }
}
