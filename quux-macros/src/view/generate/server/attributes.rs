use super::parse;
use crate::view::parse::prelude::*;
use element::Attributes;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl Attributes {
    /// Adds the scoped id attribute with the value of `id` if the containing element needs an id because it is reactive.
    /// If the element is not reactive, nothing is added.
    pub fn insert_scoped_id(&mut self, id: &str) {
        if !self.element_needs_id {
            return;
        }
        self.attributes.insert(
            "data-quux-scoped-id".to_string(),
            parse(quote! { format!("{}.{}", scope_id, #id) }),
        );
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.attributes.is_empty() {
            return tokens.extend(quote! {
                String::new()
            });
        }
        let attributes = self.attributes.iter().map(|(key, value)| {
            quote! {
                format!("{}=\"{}\"", #key, #value)
            }
        });
        tokens.extend(quote! {
            String::new() + #(&#attributes)+*
        });
    }
}
