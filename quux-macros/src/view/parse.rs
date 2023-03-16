pub use html_ident::HtmlIdent;
use internal::prelude::*;
pub use item::Item;
use syn::parse_quote;

pub mod component;
pub mod element;
mod html_ident;
mod item;

#[derive(Clone)]
pub struct View {
    pub context: Expr,
    pub element: Element,
    pub component_enum: Type,
}

impl Parse for View {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let context = input.parse()?;
        input.parse::<Token![,]>()?;
        // let component_enum = input.parse()?;
        // input.parse::<Token![,]>()?;
        let element = input.parse()?;
        Ok(Self {
            context,
            element,
            component_enum: parse_quote!(_),
        })
    }
}

pub mod prelude {
    pub use super::{
        component::{self, Component},
        element::prelude::*,
        HtmlIdent, Item, View,
    };
}

mod internal {
    pub mod prelude {
        pub use super::super::prelude::*;
        pub use quote::ToTokens;
        pub use syn::{
            braced, parenthesized,
            parse::{Parse, ParseStream},
            token::{Brace, Paren},
            Expr, Ident, LitInt, Pat, Path, Token, Type,
        };
    }
}
