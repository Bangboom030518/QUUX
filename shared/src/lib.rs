#![warn(clippy::pedantic, clippy::nursery)]

use std::{
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
};
pub use stores::Store;
pub mod stores;
pub use cfg_if;
pub use postcard;
use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
lazy_static::lazy_static! {
    pub static ref TREE_INTERPOLATION_ID: uuid::Uuid = uuid::Uuid::new_v4();
}

// TODO: Mallory could be naughty. ID should not be global, but should be unique to each request.
static GLOBAL_ID: AtomicU64 = AtomicU64::new(0);

#[must_use]
pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(not(target_arch = "wasm32"))]
pub fn render_to_string<'a, T, P>(component: &T) -> String
where
    T: Component<'a, Props = P>,
{
    let RenderData {
        html,
        component_node,
    } = component.render();
    let bytes = postcard::to_stdvec(&component_node)
        .expect("Failed to serialize `RenderContext` (internal)");
    let component_node = base64::encode(bytes);
    format!(
        "<!DOCTYPE html>{}",
        html.replace(&TREE_INTERPOLATION_ID.to_string(), &component_node)
    )
}

#[must_use]
pub fn generate_id() -> String {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed).to_string()
}

pub struct RenderData {
    pub html: String,
    pub component_node: ClientComponentNode,
}

pub trait Component<'a>: Serialize + Deserialize<'a> {
    type Props;

    fn init(props: Self::Props) -> Self;

    fn serialize(&self) -> Vec<u8> {
        postcard::to_stdvec(&self).expect("couldn't serialize component (quux internal error)")
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render(&self) -> RenderData;

    #[cfg(target_arch = "wasm32")]
    fn render(self, context: RenderContext);

    #[must_use]
    fn from_bytes(bytes: &'a [u8]) -> Self {
        postcard::from_bytes(bytes).expect("couldn't deserialize component (quux internal error)")
    }
}

#[derive(Serialize, Deserialize)]
/// Represents a reactive node on the client. Only for `Component`s.
pub struct ClientComponentNode {
    /// The serialised component
    pub component: Vec<u8>,
    pub render_context: RenderContext,
}

impl FromStr for ClientComponentNode {
    type Err = ClientParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = base64::decode(s).map_err(ClientParseError::Base64Decode)?;
        let node = postcard::from_bytes(&bytes).map_err(ClientParseError::PostcardDecode)?;
        Ok(node)
    }
}

pub enum ClientParseError {
    Base64Decode(base64::DecodeError),
    PostcardDecode(postcard::Error),
}

impl ClientParseError {
    const BASE_64_MESSAGE: &str = "Failed to decode data as base64";
    const POSTCARD_DECODE_MESSAGE: &str = "Failed to decode bytes";
}

impl std::fmt::Display for ClientParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base64Decode(_) => write!(f, "{}", Self::BASE_64_MESSAGE),
            Self::PostcardDecode(_) => write!(f, "{}", Self::POSTCARD_DECODE_MESSAGE),
        }
    }
}

impl std::fmt::Debug for ClientParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base64Decode(err) => write!(f, "{}: {err:?}", Self::BASE_64_MESSAGE),
            Self::PostcardDecode(err) => write!(f, "{}: {err:?}", Self::POSTCARD_DECODE_MESSAGE),
        }
    }
}

impl std::error::Error for ClientParseError {}

/// The id is passed to render method on client
/// Children are recusively hydrated
/// This created whenever a `view!()` macro is used
///
/// For an `view!()`, this will contain an id used on the client for reactivity, as well as any children that are components.
/// This will allow for a `view!()` to manage its children by encapsulating them under one unique id.
#[derive(Serialize, Deserialize, Default)]
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
            component_node: ClientComponentNode {
                component: postcard::to_stdvec(&self).expect("Failed to serialize component (quux internal error)"),
                render_context: RenderContext::default()
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn render(self, _: RenderContext) {}
}

#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn get_reactive_element(scope_id: &str, scoped_id: &str) -> web_sys::Element {
    get_document()
        .query_selector(&format!(
            "[data-quux-scope-id='{scope_id}'] [data-quux-scoped-id='{scoped_id}']"
        ))
        .expect("Failed to get element with scoped id (quux internal error)")
        .expect("Failed to get element with scoped id (quux internal error)")
}

#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn get_document() -> web_sys::Document {
    web_sys::window()
        .expect("Failed to get window (quux internal error)")
        .document()
        .expect("Failed to get document (quux internal error)")
}

#[cfg(target_arch = "wasm32")]
pub fn init_app<'a, T>()
where
    T: Component<'a>,
{
    let init_script = get_document()
        .get_element_by_id("__quux_init_script__")
        .expect("`__quux_init_script__` not found");

    let tree = init_script
        .get_attribute("data-quux-tree")
        .expect("`__quux_init_script__` doesn't have a tree attached (quux internal error)");
    let tree: ClientComponentNode = tree.parse().unwrap();
    let root_component = T::from_bytes(&tree.component);
    root_component.render(tree.render_context);
}
