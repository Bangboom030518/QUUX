use super::internal::prelude::*;

#[derive(Clone)]
pub struct If {
    pub condition: Expr,
    pub true_branch: Box<Item>,
    pub false_branch: Option<Box<Item>>,
}

impl Parse for If {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![if]>()?;
        let condition = input.call(Expr::parse_without_eager_brace)?;
        let true_branch;
        braced!(true_branch in input);
        let true_branch = Box::new(true_branch.parse()?);

        let false_branch = if input.parse::<Token![else]>().is_ok() {
            let item;
            braced!(item in input);
            Some(Box::new(item.parse()?))
        } else {
            None
        };

        Ok(Self {
            condition,
            true_branch,
            false_branch,
        })
    }
}
