pub use stores::Store;
use std::sync::atomic::{AtomicU64, Ordering};

pub mod stores;

static GLOBAL_ID: AtomicU64 = AtomicU64::new(0);

pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn generate_id() -> String {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed).to_string()
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

/// Represents a reactive node on the client. Only for `Component`s. 
struct ClientComponentNode<'a> {
    component: &'a dyn Render,
    render_context: RenderContext<'a>,
    /// This is **only** for the parent to know where this child is. This child will never know its static, as it's not included in the `RenderContext`.
    static_id: &'static str,
}

/// The id is passed to render method on client
/// Children are recusively hydrated
/// This created whenever a `view!()` macro is used
/// 
/// For an `view!()`, this will contain an id used on the client for reactivity, as well as any children that are components.
/// This will allow for a `view!()` to manage its children by encapsulating them under one unique id.
pub struct RenderContext<'a> {
    children: Vec<ClientComponentNode<'a>>,
    id: String,
}
