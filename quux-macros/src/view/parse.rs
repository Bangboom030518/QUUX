use internal::prelude::*;

pub mod component;
pub mod element;

#[allow(clippy::module_name_repetitions)]
pub fn parse_html_ident(input: ParseStream) -> syn::Result<String> {
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
    Ok(result)
}

#[derive(Clone)]
pub struct View {
    pub context: Ident,
    pub element: Element,
}

impl Parse for View {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let context = input.parse()?;
        input.parse::<Token![,]>()?;
        let element = input.parse()?;
        Ok(Self { context, element })
    }
}

#[derive(Clone)]
pub enum Item {
    Component(Component),
    Element(Element),
    Expression(Expr),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            let content;
            braced!(content in input);
            return Ok(Self::Expression(content.parse()?));
        }

        if input.peek(Token![@]) {
            input.parse::<Token![@]>()?;
            return Ok(Self::Component(input.parse()?));
        }

        if input.peek(Ident) {
            return Ok(Self::Element(input.parse()?));
        }

        Err(input.error("Invalid Token :("))
    }
}

pub mod prelude {
    pub use super::{
        component::{self, Component},
        element::{self, Element},
        Item, View,
    };
}

mod internal {
    pub mod prelude {
        pub use super::super::{parse_html_ident, prelude::*};
        pub use quote::ToTokens;
        pub use syn::{
            braced, parenthesized,
            parse::{Parse, ParseStream},
            token::{Brace, Paren},
            Expr, Ident, LitInt, Pat, Path, Token,
        };
    }
}
