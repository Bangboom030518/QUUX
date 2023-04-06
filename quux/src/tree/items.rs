use crate::internal::prelude::*;

use super::BoxedComponents;

#[derive(Clone, Default)]
pub struct Items(Vec<Item>);

impl Items {
    pub fn components(&self) -> BoxedComponents {
        let components = self.0.iter().map(Item::components);
    }
}

impl Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Self(items) = self;
        for item in items {
            write!(f, "{item}")?;
        }
        Ok(())
    }
}
