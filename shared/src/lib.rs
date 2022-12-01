use std::sync::atomic::{AtomicU64, Ordering};
pub use stores::Store;
pub mod stores;
use base64::encode;
pub use postcard;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
lazy_static::lazy_static! {
    pub static ref TREE_INTERPOLATION_ID: uuid::Uuid = uuid::Uuid::new_v4();
}

static GLOBAL_ID: AtomicU64 = AtomicU64::new(0);

pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render_to_string<'a, T, P>(component: T) -> String
where
    T: Component<'a, Props = P>,
{
    let RenderData {
        html,
        render_context,
    } = component.render();
    let bytes = postcard::to_stdvec(&render_context).expect("Failed to serialize `RenderContext` (internal)");
    let render_context = encode(bytes);
    format!(
        "<!DOCTYPE html>{}",
        html.replace(&TREE_INTERPOLATION_ID.to_string(), &render_context)
    )
}

pub fn generate_id() -> String {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed).to_string()
}

pub struct RenderData {
    pub html: String,
    pub render_context: RenderContext,
}

pub trait Component<'a>: Serialize + Deserialize<'a> {
    type Props;

    fn init(props: Self::Props) -> Self;

    #[cfg(target_arch = "wasm32")]
    fn render(&self, context: RenderContext);

    #[cfg(not(target_arch = "wasm32"))]
    fn render(&self) -> RenderData;
}

#[derive(Serialize, Deserialize)]
/// Represents a reactive node on the client. Only for `Component`s.
pub struct ClientComponentNode {
    // The serialised component
    pub component: Vec<u8>,
    pub render_context: RenderContext,
    /// This is **only** for the parent to know where this child is. This child will never know its static, as it's not included in the `RenderContext`.
    pub static_id: String,
}

/// The id is passed to render method on client
/// Children are recusively hydrated
/// This created whenever a `view!()` macro is used
///
/// For an `view!()`, this will contain an id used on the client for reactivity, as well as any children that are components.
/// This will allow for a `view!()` to manage its children by encapsulating them under one unique id.
#[derive(Serialize, Deserialize)]
pub struct RenderContext {
    pub children: Vec<ClientComponentNode>,
    pub id: String,
}

pub struct EmptyProps {}

/// Put this in the root component, at the end of the body
///
/// # Example
///
/// ```
/// view! {
///     html {
///         ...
///         body {
///             ...
///             @QUUXInitialise
///         }
///     }
/// }
/// ```
#[derive(Serialize, Deserialize)]
pub struct QUUXInitialise;

impl<'a> Component<'a> for QUUXInitialise {
    type Props = EmptyProps;

    fn init(_: Self::Props) -> Self {
        Self {}
    }

    #[cfg(not(target_arch = "wasm32"))]
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

    #[cfg(target_arch = "wasm32")]
    fn render(&self, _: RenderContext) {}
}
