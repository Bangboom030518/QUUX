use crate::internal::prelude::*;
pub use output::Output;
pub use serialized_components::SerializedComponent;

mod output;
mod serialized_components;

/// On the server, context is used to generate ids for sub-components.
/// This created whenever a `view!()` macro is used.
#[derive(Serialize, Deserialize)]
pub struct ServerContext<T> {
    pub id: u64,
    // TODO: update to have own struct
    pub for_loop_id: Option<String>,
    _phantom: PhantomData<T>,
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

pub trait ClientContext {
    type Context;
}

/// The `Context` passed to the render method of a component
#[server]
pub type Context<T> = ServerContext<T>;

/// The `Context` passed to the render method of a component
#[client]
pub type Context<T: Component> = <T as ClientContext>::Context;
