use crate::internal::prelude::*;
// pub use output::Output;
pub use serialized_components::SerializedComponent;

// pub mod output;
mod serialized_components;


#[derive(Serialize, Deserialize, Clone)]
pub struct Context<T>
where
    T: Component,
{
    pub id: u64,
    pub for_loop_id: Option<String>,
    _phantom: PhantomData<T>,
}

impl<T> Context<T>
where
    T: Component,
{
    #[must_use]
    pub const fn new(id: u64, for_loop_id: Option<String>) -> Self {
        Self {
            id,
            for_loop_id,
            _phantom: PhantomData,
        }
    }
}

pub trait ComponentChildren {
    type Children: crate::tree::Children;
}
