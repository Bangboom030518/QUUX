use crate::internal::prelude::*;

/// Everything the client needs to render a component.
///
/// These are created each time every time a component is used in a view, on the server.
#[derive(Serialize, Deserialize)]
pub struct SerializedComponent<T: Component> {
    pub component: T,
    pub render_context: crate::view::Context<T>,
}

impl<T> FromStr for SerializedComponent<T>
where
    T: Component + DeserializeOwned,
{
    type Err = errors::ClientParse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = base64::decode(s).map_err(errors::ClientParse::Base64Decode)?;
        let node = postcard::from_bytes(&bytes).map_err(errors::ClientParse::PostcardDecode)?;
        Ok(node)
    }
}

impl<T> SerializePostcard for SerializedComponent<T> where T: Component {}
