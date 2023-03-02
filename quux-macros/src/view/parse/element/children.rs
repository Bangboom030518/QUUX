use super::super::internal::prelude::*;

#[derive(Clone)]
pub enum Children {
    Items(Vec<Item>),
    ReactiveStore(Box<Expr>),
    ForLoop(ForLoop),
}

impl Parse for Children {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(Self::ReactiveStore(input.parse()?))
        } else if input.peek(Token![for]) {
            Ok(Self::ForLoop(input.parse()?))
        } else {
            let mut items = Vec::new();
            while !input.is_empty() {
                items.push(input.parse()?);
            }
            Ok(Self::Items(items))
        }
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::Items(Vec::new())
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
        })
    }
}
