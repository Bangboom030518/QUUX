use crate::internal::prelude::*;
#[client]
pub use dom_representation::DomRepresentation;
pub use empty::Empty;
pub use many::Many;
pub use pair::Pair;
pub use self_closing::SelfClosing;

pub mod branch;
#[cfg(target_arch = "wasm32")]
mod dom_representation;
mod empty;
mod many;
mod pair;
mod self_closing;
mod text;

pub trait Item: Display + Debug + Sized {
    // TODO: make constants
    fn is_self_closing(&self) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        false
    }

    #[client]
    fn hydrate(&mut self) {}


    #[client]
    fn dom_representation(&mut self) -> DomRepresentation;

    // TODO: why does it skip ids?
    fn insert_id(&mut self, id: u64) -> u64;

    // fn boxed<'a>(self) -> Box<dyn Item + 'a>
    // where
    //     Self: Sized + 'a,
    // {
    //     Box::new(self)
    // }
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
