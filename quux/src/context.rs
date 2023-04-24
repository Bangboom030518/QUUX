use crate::internal::prelude::*;

#[derive(Default)]
pub struct Context<T: Component> {
    _phantom: PhantomData<T>,
}

impl<T: Component> Context<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}
