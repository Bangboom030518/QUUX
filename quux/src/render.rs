use crate::internal::prelude::*;
use std::str::FromStr;
/// The id is passed to render method on client
/// Children are recusively hydrated.
/// This created whenever a `view!()` macro is used
///
/// For an `view!()`, this will contain an id used on the client for reactivity, as well as any children that are components.
/// This will allow for a `view!()` to manage its children by encapsulating them under one unique id.
#[derive(Serialize, Deserialize)]
pub struct Context {
    pub children: Vec<ClientComponentNode>,
    pub for_loop_children: Vec<Vec<ClientComponentNode>>,
    pub id: u64,
    pub for_loop_id: Option<String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            children: Vec::new(),
            for_loop_children: Vec::new(),
            id: 0,
            for_loop_id: None,
        }
    }
}

#[server]
pub struct Output<T>
where
    T: Component + Sized,
{
    pub html: String,
    pub component_node: ClientComponentNode,
    _phantom: PhantomData<T>,
}

#[server]
impl<T> Output<T>
where
    T: Component + Sized,
{
    pub fn new(html: &str, component_node: ClientComponentNode) -> Self {
        Self {
            html: html.to_string(),
            component_node,
            _phantom: PhantomData,
        }
    }
}

#[client]
#[derive(Default)]
pub struct Output<T>
where
    T: Component + Sized,
{
    pub component: T,
}

#[client]
impl Output {
    pub fn new(component: T) {
        Self { component }
    }
}

#[derive(Serialize, Deserialize)]
/// Represents a reactive node on the client. Only for `Component`s.
pub struct ClientComponentNode {
    pub component: Box<dyn std::any::Any>,
    // #[serde(bound(deserialize = "T: Deserialize<'a>"))]
    pub render_context: Context,
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
