use crate::internal::prelude::*;
pub use output::Output;
pub use serialized_components::SerializedComponent;

pub mod output;
mod serialized_components;

/// On the server, context is used to generate ids for sub-components.
/// This created whenever a `view!()` macro is used.
#[derive(Serialize, Deserialize, Clone)]
pub struct ServerContext<T> {
    pub id: u64,
    // TODO: update to have own struct
    pub for_loop_id: Option<String>,
    _phantom: PhantomData<T>,
}

impl<T> Default for ServerContext<T> {
    fn default() -> Self {
        Self::new(0, None)
    }
}

impl<T> ServerContext<T> {
    #[must_use]
    pub const fn new(id: u64, for_loop_id: Option<String>) -> Self {
        Self {
            id,
            for_loop_id,
            _phantom: PhantomData,
        }
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ClientContext<T>
where
    T: Component,
{
    pub id: u64,
    pub for_loop_id: Option<String>,
    pub components: <T as ComponentChildren>::Components,
    pub for_loop_components: <T as ComponentChildren>::ForLoopComponents,
}

impl<T> ClientContext<T>
where
    T: Component,
{
    pub const fn new(
        id: u64,
        for_loop_id: Option<String>,
        components: <T as ComponentChildren>::Components,
        for_loop_components: <T as ComponentChildren>::ForLoopComponents,
    ) -> Self {
        Self {
            id,
            for_loop_id,
            components,
            for_loop_components,
        }
    }
}

pub trait ComponentChildren {
    type Components: Serialize + DeserializeOwned + Clone;
    type ForLoopComponents: Serialize + DeserializeOwned + Clone;
}

/// The `Context` passed to the render method of a component
#[server]
pub type Context<T> = ServerContext<T>;

/// The `Context` passed to the render method of a component
#[client]
pub type Context<T> = ClientContext<T>;
