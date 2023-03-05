use super::super::GLOBAL_ID;
use crate::view::parse::{
    element::{
        children::{Items, ReactiveStore},
        GenerationData,
    },
    prelude::*,
};
use element::Children;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::sync::atomic::Ordering::Relaxed;

impl From<ReactiveStore> for TokenStream {
    /// Generates the body of an element.
    fn from(ReactiveStore(store): ReactiveStore) -> Self {
        quote! { #store.get() }
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let GenerationData { html } = self.clone().into();
        tokens.extend(quote! { &#html });
    }
}

impl Items {
    /// Generates the body of an element.
    pub fn html_body_tokens(&self) -> TokenStream {
        if self.items.is_empty() {
            return quote! {
                String::new()
            };
        }
        let html = &self.items;
        quote! {
            String::new() + #(#html)+*
        }
    }
}

impl Element {
    const SELF_CLOSING_ELEMENTS: &'static [&'static str] = &[
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "source", "source",
        "track", "wbr",
    ];

    fn is_self_closing(&self) -> bool {
        Self::SELF_CLOSING_ELEMENTS.contains(&self.tag_name.to_lowercase().as_str())
    }

    /// Generates the html body for an element.
    /// Sets `self.attributes.element_needs_id` if necessary
    fn html_body_tokens(&mut self) -> TokenStream {
        if !matches!(&self.children, Children::Items(items) if items.items.is_empty()) {
            assert!(
                !self.is_self_closing(),
                "Self-closing elements cannot contain children"
            );
        }
        match self.children.clone() {
            Children::Items(items) => items.html_body_tokens(),
            Children::ReactiveStore(store) => {
                self.attributes.element_needs_id = true;
                store.into()
            }
            Children::ForLoop(for_loop) => for_loop.into(),
        }
    }
}

impl From<Element> for GenerationData {
    fn from(mut value: Element) -> Self {
        let attributes = TokenStream::from(value.attributes.clone());
        let tag_name = value.tag_name.clone();
        if value.is_self_closing() {
            Self {
                html: quote! {
                    format!("<{} {} />", #tag_name, #attributes)
                },
                // ..value.component_initialisation_code
            }
        } else {
            value
                .attributes
                .insert_scoped_id(&GLOBAL_ID.fetch_add(1, Relaxed).to_string());
            let body = value.html_body_tokens();
            Self {
                html: quote! {
                    format!("<{0} {1}>{2}</{0}>", #tag_name, #attributes, #body)
                },
                // ..value.component_initialisation_code
            }
        }
    }
}
