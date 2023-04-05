use super::super::internal::{self, prelude::*};
use crate::view::generate::Html;

pub use for_loop::ForLoop;
pub use if_expr::If;
pub use match_expr::Match;
pub use reactive_store::ReactiveStore;

pub mod for_loop;
pub mod if_expr;
pub mod match_expr;
pub mod reactive_store;

#[derive(Clone, Default)]
pub struct Items {
    pub items: Vec<Item>,
    pub html: Html,
}

impl Parse for Items {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(Self {
            items,
            ..Default::default()
        })
    }
}

#[derive(Clone)]
pub enum Children {
    Items(Items),
    ReactiveStore(ReactiveStore),
    ForLoop(ForLoop),
    If(If),
    Match(Match),
}

impl Children {
    pub fn parse(input: ParseStream, id: u64) -> syn::Result<Self> {
        if !input.peek(Brace) {
            return Ok(Self::default());
        }
        let children;

        braced!(children in input);

        if children.peek(Token![$]) {
            return Ok(Self::ReactiveStore(children.parse()?));
        }

        if children.peek(Token![for]) {
            return Ok(Self::ForLoop(ForLoop::parse(&children, id)?));
        }

        if children.peek(Token![if]) {
            return Ok(Self::If(children.parse()?));
        }

        if children.peek(Token![match]) {
            return Ok(Self::Match(children.parse()?));
        }

        Ok(Self::Items(children.parse()?))
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::Items(Items::default())
    }
}
