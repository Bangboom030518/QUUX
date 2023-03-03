use super::parse;
use crate::view::parse::prelude::*;
use element::Attributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

impl Attributes {
    /// Adds the scoped id attribute with the value of `id` if the containing element needs an id because it is reactive.
    /// If the element is not reactive, nothing is added.
    pub fn insert_scoped_id(&mut self, id: &str) -> Option<Expr> {
        if self.element_needs_id {
            self.attributes.insert(
                "data-quux-scoped-id".to_string(),
                parse(quote! { format!("{}.{}", scope_id, #id) }),
            )
        } else {
            None
        }
    }
}

impl From<Attributes> for TokenStream {
    fn from(Attributes { attributes, .. }: Attributes) -> TokenStream {
        let attributes = attributes.into_iter().map(|(key, value)| {
            quote! {
                format!("{}=\"{}\"", #key, #value)
            }
        });
        quote! {
            String::new() + #(&#attributes)+*
        }
    }
}
