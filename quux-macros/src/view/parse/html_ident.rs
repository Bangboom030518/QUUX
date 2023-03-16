use syn::LitStr;

use super::internal::prelude::*;
use std::ops::Deref;

// TODO: make expr?
#[derive(Clone, Default)]
pub struct HtmlIdent(pub String);

impl Parse for HtmlIdent {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(LitStr) {
            return Ok(Self(input.parse::<LitStr>()?.value()))
        }

        let mut result = input.parse::<Ident>()?.to_string();

        if input.peek(Ident) {
            return Err(input.error("unexpected whitespace in html identifier"));
        }

        while !input.is_empty() {
            if input.peek(Token![-]) {
                input.parse::<Token![-]>()?;
                result += "-";
                continue;
            }

            if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                result += ":";
                continue;
            }

            if input.peek(Token![.]) {
                input.parse::<Token![.]>()?;
                result += ".";
                continue;
            }

            if input.peek(LitInt) {
                result += &input.parse::<LitInt>()?.to_string();
                continue;
            }

            if input.peek(Ident) && !input.peek2(Ident) {
                result += &input.parse::<Ident>()?.to_string();
                continue;
            }
            break;
        }
        Ok(Self(result))
    }
}

impl Deref for HtmlIdent {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToTokens for HtmlIdent {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
