use std::collections::HashMap;
pub use stores::Store;

pub mod stores;

pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn init_app<T, P>(_component: T)
where
    T: Component<Props = P>,
{
    todo!("Implement `init_app`");
}

pub struct RenderData<'a> {
    pub html: String,
    pub render_context: RenderContext<'a>,
}

pub trait Render {
    #[cfg(target = "wasm")]
    fn render(&self, context: RenderContext);

    #[cfg(not(target = "wasm"))]
    fn render(&self) -> RenderData;
}

pub trait Component: Render {
    type Props;

    fn init(props: Self::Props) -> Self;
}

struct ClientNode<'a> {
    component: &'a dyn Render,
    render_context: RenderContext<'a>,
}

/// The id is passed to render method on client
/// Children are recusively hydrated
/// This created whenever a `view!()` macro is used
pub struct RenderContext<'a> {
    children: Vec<ClientNode<'a>>,
    id: String,
}
