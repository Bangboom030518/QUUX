pub use html_ident::HtmlIdent;
use internal::prelude::*;
pub use item::Item;

pub mod component;
pub mod element;
mod html_ident;
mod item;

#[derive(Clone)]
pub struct View {
    pub context: Expr,
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
            punctuated::Punctuated,
            token::{Brace, Paren},
            Expr, Ident, LitInt, Pat, Path, Token, Type, TypePath,
        };
    }
}
