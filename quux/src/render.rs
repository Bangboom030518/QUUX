use crate::internal::prelude::*;
use std::str::FromStr;

/// The id is passed to render method on client
/// Children are recusively hydrated
/// This created whenever a `view!()` macro is used
///
/// For an `view!()`, this will contain an id used on the client for reactivity, as well as any children that are components.
/// This will allow for a `view!()` to manage its children by encapsulating them under one unique id.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Context<T>
where
    T: component::Enum,
{
    pub children: Vec<ClientComponentNode<T>>,
    pub for_loop_children: Vec<Vec<ClientComponentNode<T>>>,
    pub id: u64,
    pub for_loop_id: Option<String>,
}

impl<T: component::Enum> Default for Context<T> {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            for_loop_children: Vec::new(),
            id: 0,
            for_loop_id: None,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub struct Output<T>
where
    T: component::Enum,
{
    pub html: String,
    pub component_node: ClientComponentNode<T>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
pub struct Output<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[cfg(target_arch = "wasm32")]
impl<T> Output<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Represents a reactive node on the client. Only for `Component`s.
pub struct ClientComponentNode<T>
where
    T: component::Enum,
{
    pub component: T,
    // #[serde(bound(deserialize = "T: Deserialize<'a>"))]
    pub render_context: Context<T>,
}

impl<T> FromStr for ClientComponentNode<T>
where
    T: component::Enum + DeserializeOwned,
{
    type Err = errors::ClientParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = base64::decode(s).map_err(errors::ClientParse::Base64Decode)?;
        let node = postcard::from_bytes(&bytes).map_err(errors::ClientParse::PostcardDecode)?;
        Ok(node)
    }
}

impl<T> SerializePostcard for ClientComponentNode<T> where T: component::Enum {}
