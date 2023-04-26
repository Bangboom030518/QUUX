use crate::internal::prelude::*;

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
}
