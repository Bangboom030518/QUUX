use std::sync::atomic::{AtomicU64, Ordering};
pub use stores::Store;
pub mod stores;
use deku::prelude::*;
use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static! {
    pub static ref TREE_INTERPOLATION_ID: Uuid = Uuid::new_v4();
}

static GLOBAL_ID: AtomicU64 = AtomicU64::new(0);

pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub fn render_to_string<T, P>(component: T) -> String
where
    T: Component<Props = P>,
{
    let RenderData {
        html,
        render_context,
    } = component.render();
    let render_context = render_context;
    format!(
        "<!DOCTYPE html>{}",
        html.replace(&TREE_INTERPOLATION_ID.to_string(), render_context)
    )
}

pub fn generate_id() -> String {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed).to_string()
}

pub struct RenderData {
    pub html: String,
    pub render_context: RenderContext,
}

pub trait Render: DekuRead<'static> + DekuWrite {
    #[cfg(target_arch = "wasm32")]
    fn render(&self, context: RenderContext);

    #[cfg(not(target_arch = "wasm32"))]
    fn render(&self) -> RenderData;
}

pub trait Component: Render {
    type Props;

    fn init(props: Self::Props) -> Self;
}

#[derive(DekuRead, DekuWrite)]
/// Represents a reactive node on the client. Only for `Component`s.
pub struct ClientComponentNode {
    pub component: Box<dyn Render>,
    pub render_context: RenderContext,
    /// This is **only** for the parent to know where this child is. This child will never know its static, as it's not included in the `RenderContext`.
    pub static_id: &'static str,
}

/// The id is passed to render method on client
/// Children are recusively hydrated
/// This created whenever a `view!()` macro is used
///
/// For an `view!()`, this will contain an id used on the client for reactivity, as well as any children that are components.
/// This will allow for a `view!()` to manage its children by encapsulating them under one unique id.
#[derive(DekuRead, DekuWrite)]
pub struct RenderContext {
    pub children: Vec<ClientComponentNode>,
    pub id: String,
}

/// Put this in the root component, at the end of the body
///
/// # Example
///
/// ```
/// view! {
///     html(lang="en") {
///         head {
///             title {
///                 { "My App" }
///             }
///         }
///         body {
///             button {
///                 { self.count }
///             }
///             @QUUXInitialise
///         }
///     }
/// }
/// ```

pub struct EmptyProps {}

pub struct QUUXInitialise;

impl Component for QUUXInitialise {
    type Props = EmptyProps;

    fn init(_: Self::Props) -> Self {
        Self {}
    }
}

impl Render for QUUXInitialise {
    fn render(&self) -> RenderData {
        RenderData {
            html: format!(
                "<script type=\"module\" id=\"__quux_init_script__\" data-quux-tree=\"{}\">{}; await init('./assets/quux_bg.wasm')</script>",
                *TREE_INTERPOLATION_ID,
                include_str!("../../assets/quux.js"),
            ),
            render_context: RenderContext {
                children: Vec::new(),
                id: String::new(),
            },
        }
    }
}
