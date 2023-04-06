use super::internal::prelude::*;

#[derive(Clone)]
pub struct ReactiveStore(pub Box<Expr>);

impl Parse for ReactiveStore {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![$]>()?;
        Ok(Self(Box::new(input.parse()?)))
    }
}
