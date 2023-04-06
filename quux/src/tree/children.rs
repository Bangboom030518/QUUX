use crate::internal::prelude::*;
use super::{DisplayStore, BoxedComponents};

#[derive(Clone)]
pub enum Children {
    Items(Items),
    ReactiveStore(DisplayStore),
    SelfClosing, // ForLoop(ForLoop),
}

impl Display for Children {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Items(items) => write!(f, "{items}"),
            Self::ReactiveStore(store) => write!(f, "{}", store.get()),
            Self::SelfClosing => Ok(()),
        }
    }
}

fn components() -> impl Components {
}

impl Children {
    pub const fn is_self_closing(&self) -> bool {
        matches!(self, Self::SelfClosing)
    }

    pub fn components(&self) -> BoxedComponents {
        match self {
            Children::Items(items) => Box::new(items.components()),
            Children::ReactiveStore(_) | Children::SelfClosing => Box::new(()),
        }
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::Items(Items::default())
    }
}
