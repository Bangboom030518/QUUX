use crate::internal::prelude::*;

#[derive(Clone, Debug)]
pub struct Empty;

impl Display for Empty {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Hydrate for Empty {}

#[client]
impl From<Empty> for DomRepresentation {
    fn from(_: Empty) -> Self {
        Self::None
    }
}

impl Item for Empty {
    fn is_empty(&self) -> bool {
        true
    }

    fn insert_id(&mut self, id: u64) -> u64 {
        id
    }
}
