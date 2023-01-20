use super::super::internal::prelude::*;

#[derive(Clone)]
pub struct Attribute {
    pub key: String,
    pub value: Value,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = parse_html_ident(input)?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { key, value })
    }
}

#[derive(Clone)]
pub enum Value {
    Reactive(Expr),
    Static(Expr),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(Self::Reactive(input.parse()?))
        } else {
            Ok(Self::Static(input.parse()?))
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reactive(expr) => write!(f, "${}", expr.to_token_stream()),
            Self::Static(expr) => write!(f, "{}", expr.to_token_stream()),
        }
    }
}
