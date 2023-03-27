use super::super::internal::prelude::*;
use crate::view::generate::Html;

#[derive(Clone)]
pub struct ReactiveStore(pub Box<Expr>);

impl Parse for ReactiveStore {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![$]>()?;
        Ok(Self(Box::new(input.parse()?)))
    }
}

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
}

impl Children {
    pub fn parse(input: ParseStream, id: u64) -> syn::Result<Self> {
        let children;
        braced!(children in input);
        if children.peek(Token![$]) {
            return Ok(Self::ReactiveStore(children.parse()?));
        }

        if children.peek(Token![for]) {
            return Ok(Self::ForLoop(ForLoop::parse(&children, id)?));
        }

        Ok(Self::Items(children.parse()?))
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::Items(Items::default())
    }
}

#[derive(Clone)]
pub enum ForLoopIterable {
    Reactive(Expr),
    Static(Expr),
}

#[derive(Clone)]
pub struct ForLoop {
    pub pattern: Pat,
    pub iterable: ForLoopIterable,
    pub item: Box<Item>,
    pub bindings: Vec<Ident>,
    pub id: u64,
}

impl ForLoop {
    pub const fn is_reactive(&self) -> bool {
        matches!(self.iterable, ForLoopIterable::Reactive(_))
    }

    pub fn parse(input: ParseStream, id: u64) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pattern = input.parse()?;
        input.parse::<Token![in]>()?;
        let iterable = if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            ForLoopIterable::Reactive
        } else {
            ForLoopIterable::Static
        }(input.call(Expr::parse_without_eager_brace)?);
        let item;
        braced!(item in input);
        let item = item.parse()?;
        Ok(Self {
            pattern,
            iterable,
            item: Box::new(item),
            bindings: Vec::new(),
            id,
        })
    }
}
