// use syn::punctuated::Punctuated;
use super::internal::prelude::*;

#[derive(Clone)]
pub struct Arm {
    pub pattern: Pat,
    pub item: Items,
}

impl Parse for Arm {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pattern = input.parse()?;
        input.parse::<Token![=>]>()?;
        let item;
        braced!(item in input);
        let item = item.parse()?;
        Ok(Self { pattern, item })
    }
}

#[derive(Clone)]
pub struct Match {
    pub scrutinee: Expr,
    pub arms: Vec<Arm>,
}

impl Parse for Match {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![match]>()?;
        let scrutinee = input.call(Expr::parse_without_eager_brace)?;
        let arms;
        braced!(arms in input);
        let arms = Punctuated::<Arm, Token![,]>::parse_terminated(&arms)?
            .into_iter()
            .collect();
        Ok(Self { scrutinee, arms })
    }
}
