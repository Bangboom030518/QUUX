use super::Hydrate;
use crate::internal::prelude::*;
pub use empty::Empty;
pub use many::Many;
pub use pair::Pair;
pub use self_closing::SelfClosing;

pub mod branch;
mod empty;
mod many;
mod pair;
mod self_closing;

impl Item for String {}

impl Hydrate for String {}

pub trait Item: Display + Hydrate {
    // TODO: make constants?
    fn is_self_closing(&self) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn boxed<'a>(self) -> Box<dyn Item + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

impl<T: Display> Hydrate for Store<T> {
    fn hydrate(self) {
        todo!()
    }
}

// TODO: allow n-length tuple
#[allow(clippy::missing_const_for_fn)]
pub fn children<A, B>(children: (A, B)) -> Pair<A, B>
where
    A: Item,
    B: Item,
{
    Pair(children.0, children.1)
}
