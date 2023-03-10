use super::super::internal::prelude::*;
use crate::view::generate::server::Html;

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
    pub component_initialisation_code: Html,
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

impl Parse for Children {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![$]) {
            return Ok(Self::ReactiveStore(input.parse()?));
        }

        if input.peek(Token![for]) {
            return Ok(Self::ForLoop(input.parse()?));
        }

        Ok(Self::Items(input.parse()?))
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
    pub binding: Option<Ident>,
}

impl ForLoop {
    pub const fn is_reactive(&self) -> bool {
        matches!(self.iterable, ForLoopIterable::Reactive(_))
    }
}

impl Parse for ForLoop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
        Ok(Self {
            pattern,
            iterable,
            item: Box::new(item.parse()?),
            binding: None,
        })
    }
}
