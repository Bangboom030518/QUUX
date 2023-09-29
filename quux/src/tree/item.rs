use crate::internal::prelude::*;
#[cfg_client]
pub use dom_representation::DomRepresentation;
pub use empty::Empty;
pub use many::Many;
pub use pair::Pair;
pub use raw_html::RawHtml;
pub use self_closing::SelfClosing;
pub use text::Text;

pub mod branch;
#[cfg(target_arch = "wasm32")]
mod dom_representation;
mod empty;
mod many;
mod pair;
mod raw_html;
mod self_closing;
mod text;

pub trait Item: Display + Debug + Sized {
    const IS_SELF_CLOSING: bool = false;
    const IS_EMPTY: bool = false;

    #[cfg_client]
    fn hydrate(&mut self) {}

    #[cfg_client]
    fn dom_representation(&mut self) -> DomRepresentation;

    // TODO: why does it skip ids?
    fn insert_id(&mut self, id: u64) -> u64;
}

#[allow(clippy::missing_const_for_fn)]
pub fn children<A, B>(children: (A, B)) -> Pair<A, B>
where
    A: Item,
    B: Item,
{
    Pair(children.0, children.1)
}
