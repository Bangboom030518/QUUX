use crate::view::parse::prelude::*;
use element::Attributes;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_quote;

impl Attributes {
    /// Adds the scoped id attribute with the value of `id` if the containing element needs an id because it is reactive.
    /// If the element is not reactive, nothing is added.
    fn insert_scoped_id(&mut self) {
        if !self.element_needs_id {
            return;
        }
        let id = self.id;
        self.attributes.insert(
            "data-quux-id".to_string(),
            parse_quote!(format!("{}.{}", &id, #id)),
        );
    }
}

impl ToTokens for Attributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut attributes = self.clone();

        attributes.insert_scoped_id();

        if attributes.attributes.is_empty() {
            return tokens.extend(quote! {
                String::new()
            });
        }

        let attributes = attributes.attributes.iter().map(|(key, value)| {
            quote! {
                format!("{}=\"{}\"", #key, #value)
            }
        });
        let for_loop_id = if self.is_root {
            quote! {
                &if let Some(id) = for_loop_id {
                    format!("data-quux-for-id=\"{}\"", id)
                } else {
                    String::new()
                }
            }
        } else {
            quote! {
                ""
            }
        };

        tokens.extend(quote! {
            String::new() + #(&#attributes)+* + #for_loop_id
        });
    }
}
