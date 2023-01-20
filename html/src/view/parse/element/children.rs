use super::super::internal::prelude::*;

#[derive(Clone)]
pub enum Children {
    Children(Vec<Item>),
    ReactiveStore(Box<Expr>),
    ForLoop(ForLoop)
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
            Ok(Self::Children(items))
        }
    }
}

impl Default for Children {
    fn default() -> Self {
        Self::Children(Vec::new())
    }
}

#[derive(Clone)]
pub struct ForLoop {
    pattern: Pat,
    iterable: Expr,
    item: Box<Ident>,
}

impl Parse for ForLoop {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let parenthesized_buffer;
        parenthesized!(parenthesized_buffer in input);
        let pattern = parenthesized_buffer.parse()?;
        parenthesized_buffer.parse::<Token![in]>()?;
        let iterable: Expr = parenthesized_buffer.parse()?;
        let item;
        braced!(item in input);
        // let item = Box::new(item.parse()?);

        Ok(Self {
            pattern,
            iterable,
            item: Box::new(item.parse()?),
        })
    }
}
