use super::internal::prelude::*;

#[derive(Clone)]
pub enum Iterable {
    Reactive(Expr),
    Static(Expr),
}

#[derive(Clone)]
pub struct ForLoop {
    pub pattern: Pat,
    pub iterable: Iterable,
    pub item: Box<Item>,
    pub bindings: Vec<Ident>,
    pub id: u64,
}

impl ForLoop {
    pub const fn is_reactive(&self) -> bool {
        matches!(self.iterable, Iterable::Reactive(_))
    }

    pub fn parse(input: ParseStream, id: u64) -> syn::Result<Self> {
        input.parse::<Token![for]>()?;
        let pattern = input.parse()?;
        input.parse::<Token![in]>()?;
        let iterable = if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Iterable::Reactive
        } else {
            Iterable::Static
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
