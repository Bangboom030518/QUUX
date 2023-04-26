use crate::internal::prelude::*;

pub struct Empty;

impl Display for Empty {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Hydrate for Empty {}

impl Item for Empty {
    fn is_empty(&self) -> bool {
        true
    }
}
