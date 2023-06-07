use crate::internal::prelude::*;

#[derive(Clone, Debug)]
pub struct SelfClosing;

impl Display for SelfClosing {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Item for SelfClosing {
    const IS_SELF_CLOSING: bool = true;

    fn insert_id(&mut self, id: u64) -> u64 {
        id
    }

    #[cfg_client]
    fn dom_representation(&mut self) -> DomRepresentation {
        DomRepresentation::None
    }
}
