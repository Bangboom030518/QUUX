#![warn(clippy::pedantic, clippy::nursery)]

pub use cfg_if;
use errors::MapInternal;
pub use postcard;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
};
pub use stores::Store;

pub mod errors;
pub mod stores;

#[cfg(target_arch = "wasm32")]
pub mod dom;

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

#[must_use]
pub fn generate_id() -> String {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed).to_string()
}

pub trait SerializePostcard: Serialize {
    fn serialize_bytes(&self) -> Vec<u8> {
        postcard::to_stdvec(self).expect_internal("serialize struct")
    }

    fn serialize_base64(&self) -> String {
        let bytes = self.serialize_bytes();
        base64::encode(bytes)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct RenderData {
    pub html: String,
    pub component_node: ClientComponentNode,
}

#[cfg(target_arch = "wasm32")]
pub type RenderData = ();

pub trait Component: Serialize + DeserializeOwned {
    type Props;

    fn init(props: Self::Props) -> Self;

    #[cfg(not(target_arch = "wasm32"))]
    fn render_to_string(&self) -> String {
        let RenderData {
            html,
            component_node,
        } = self.render(RenderContext::default());
        let bytes =
            postcard::to_stdvec(&component_node).expect_internal("serialize `RenderContext`");
        let component_node = base64::encode(bytes);
        format!(
            "<!DOCTYPE html>{}",
            html.replace(&TREE_INTERPOLATION_ID.to_string(), &component_node)
        )
    }

    fn render(&self, context: RenderContext) -> RenderData;

    #[must_use]
    fn from_bytes(bytes: &[u8]) -> Self {
        postcard::from_bytes(bytes).expect("couldn't deserialize component (quux internal error)")
    }

    #[cfg(target_arch = "wasm32")]
    fn init_as_root() -> Self {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let init_script = dom::get_document()
            .get_element_by_id("__quux_init_script__")
            .expect("`__quux_init_script__` not found");

        let tree = init_script
            .get_attribute("data-quux-tree")
            .expect("`__quux_init_script__` doesn't have a tree attached");
        let tree: ClientComponentNode = tree.parse().unwrap();
        let root_component = Self::from_bytes(&tree.component);
        root_component.render(tree.render_context);
        root_component
    }
}

impl<T: Component> SerializePostcard for T {}

#[derive(Serialize, Deserialize, Clone, Default)]
/// Represents a reactive node on the client. Only for `Component`s.
pub struct ClientComponentNode {
    /// The serialised component
    pub component: Vec<u8>,
    pub render_context: RenderContext,
}

impl FromStr for ClientComponentNode {
    type Err = errors::ClientParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = base64::decode(s).map_err(errors::ClientParse::Base64Decode)?;
        let node = postcard::from_bytes(&bytes).map_err(errors::ClientParse::PostcardDecode)?;
        Ok(node)
    }
}

impl SerializePostcard for ClientComponentNode {}

/// The id is passed to render method on client
/// Children are recusively hydrated
/// This created whenever a `view!()` macro is used
///
/// For an `view!()`, this will contain an id used on the client for reactivity, as well as any children that are components.
/// This will allow for a `view!()` to manage its children by encapsulating them under one unique id.
#[derive(Serialize, Deserialize, Clone)]
pub struct RenderContext {
    pub children: Vec<ClientComponentNode>,
    pub id: String,
}

impl Default for RenderContext {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            id: generate_id()
        }
    }
}

pub struct QUUXInitialiseProps {
    pub init_script_content: &'static str,
}

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
pub struct QUUXInitialise {
    #[serde(skip)]
    init_script_content: &'static str,
}

impl Component for QUUXInitialise {
    type Props = QUUXInitialiseProps;

    fn init(
        Self::Props {
            init_script_content,
        }: Self::Props,
    ) -> Self {
        Self {
            init_script_content,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render(&self, _: RenderContext) -> RenderData {
        RenderData {
            html: format!(
                "<script type=\"module\" id=\"__quux_init_script__\" data-quux-tree=\"{}\">{};</script>",
                *TREE_INTERPOLATION_ID,
                self.init_script_content,
            ),
            component_node: ClientComponentNode {
                component: self.serialize_bytes(),
                render_context: RenderContext::default()
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn render(&self, _: RenderContext) {}
}
