use crate::internal::prelude::*;

#[derive(Debug)]
pub struct SelfClosing;

impl Display for SelfClosing {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Hydrate for SelfClosing {}

impl Item for SelfClosing {
    fn is_self_closing(&self) -> bool {
        true
    }

    fn insert_id(&mut self, id: u64) -> u64 {
        id
    }
}
