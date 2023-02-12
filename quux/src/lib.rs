// TODO: add component enum instead of trait, should auto generate

#![warn(clippy::pedantic, clippy::nursery)]

pub use cfg_if;
use errors::MapInternal;
pub use postcard;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use std::{
    str::FromStr,
    sync::atomic::{AtomicU64, Ordering},
};
pub use stores::Store;

pub mod errors;
pub mod quux_initialise;
pub mod stores;
pub use quux_initialise::QUUXInitialise;

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
pub struct RenderData<T>
where
    T: ComponentEnum,
{
    pub html: String,
    pub component_node: ClientComponentNode<T>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
pub struct RenderData<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[cfg(target_arch = "wasm32")]
impl<T> RenderData<T> {
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

pub trait Component: Serialize + DeserializeOwned {
    type Props;
    type ComponentEnum: ComponentEnum;

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

    // TODO: gobble gobble gobble
    fn render(
        &self,
        context: RenderContext<Self::ComponentEnum>,
    ) -> RenderData<Self::ComponentEnum>;

    // #[must_use]
    // fn from_bytes(bytes: &[u8]) -> Self {
    //     postcard::from_bytes(bytes).expect("couldn't deserialize component (quux internal error)")
    // }

    // TODO: doesn't need to be associated with this trait
    #[cfg(target_arch = "wasm32")]
    fn init_as_root<T: ComponentEnum + DeserializeOwned>() {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let init_script = dom::get_document()
            .get_element_by_id("__quux_init_script__")
            .expect("`__quux_init_script__` not found");

        let tree = init_script
            .get_attribute("data-quux-tree")
            .expect("`__quux_init_script__` doesn't have a tree attached");
        let tree: ClientComponentNode<T> = tree.parse().unwrap();

        // Don't deserialize here!
        // let root_component = Self::from_bytes(&tree.component);
        tree.component.render(tree.render_context);
    }
}

impl<T: Component> SerializePostcard for T {}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Represents a reactive node on the client. Only for `Component`s.
pub struct ClientComponentNode<T>
where
    T: ComponentEnum,
{
    pub component: T,
    // #[serde(bound(deserialize = "T: Deserialize<'a>"))]
    pub render_context: RenderContext<T>,
}

impl<T> FromStr for ClientComponentNode<T>
where
    T: ComponentEnum + DeserializeOwned,
{
    type Err = errors::ClientParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = base64::decode(s).map_err(errors::ClientParse::Base64Decode)?;
        let node = postcard::from_bytes(&bytes).map_err(errors::ClientParse::PostcardDecode)?;
        Ok(node)
    }
}

impl<T> SerializePostcard for ClientComponentNode<T> where T: ComponentEnum {}

pub trait ComponentEnum: Serialize + Debug + Clone + From<QUUXInitialise<Self>> {
    fn render(&self, context: RenderContext<Self>) -> RenderData<Self>;
}

/// The id is passed to render method on client
/// Children are recusively hydrated
/// This created whenever a `view!()` macro is used
///
/// For an `view!()`, this will contain an id used on the client for reactivity, as well as any children that are components.
/// This will allow for a `view!()` to manage its children by encapsulating them under one unique id.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderContext<T>
where
    T: ComponentEnum,
{
    pub children: Vec<ClientComponentNode<T>>,
    pub for_loop_children: Vec<Vec<ClientComponentNode<T>>>,
    pub id: String,
}

impl<T> Default for RenderContext<T>
where
    T: ComponentEnum,
{
    fn default() -> Self {
        Self {
            children: Vec::new(),
            for_loop_children: Vec::new(),
            id: generate_id(),
        }
    }
}

pub mod prelude {
    pub use super::{ClientComponentNode, Component, ComponentEnum, RenderContext, RenderData};
    pub use quux_macros::view;
}
