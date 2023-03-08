use super::{super::GLOBAL_ID, Html};
use crate::view::parse::{
    element::children::{Items, ReactiveStore},
    prelude::*,
};
use element::Children;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::sync::atomic::Ordering::Relaxed;
use syn::Expr;

impl ToTokens for ReactiveStore {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(store) = self;
        tokens.extend(quote! { #store.get() });
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Html(html) = self.clone().into();
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

const SELF_CLOSING_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "source", "source",
    "track", "wbr",
];

fn is_self_closing(tag_name: &str) -> bool {
    SELF_CLOSING_ELEMENTS.contains(&tag_name.to_lowercase().as_str())
}

impl Element {
    /// Generates the html body for an element.
    /// Sets `self.attributes.element_needs_id` if necessary
    fn html_body_tokens(&mut self) -> TokenStream {
        if !matches!(&self.children, Children::Items(items) if items.items.is_empty()) {
            assert!(
                !is_self_closing(&self.tag_name),
                "Self-closing elements cannot contain children"
            );
        }
        match self.children.clone() {
            Children::Items(items) => items.html_body_tokens(),
            Children::ReactiveStore(store) => {
                self.attributes.element_needs_id = true;
                store.to_token_stream()
            }
            Children::ForLoop(for_loop) => for_loop.to_token_stream(),
        }
    }

    pub fn insert_attribute(&mut self, key: &str, value: Expr) -> Option<Expr> {
        self.attributes.insert_static(key, value)
    }
}

impl From<Element> for Html {
    fn from(mut value: Element) -> Self {
        let mut attributes = value.attributes.clone();
        let tag_name = value.tag_name.clone();
        attributes.insert_scoped_id(&GLOBAL_ID.fetch_add(1, Relaxed).to_string());

        if is_self_closing(&tag_name) {
            Self(quote! {
                format!("<{} {} />", #tag_name, #attributes)
            })
        } else {
            let body = value.html_body_tokens();
            Self(quote! {
                format!("<{0} {1}>{2}</{0}>", #tag_name, #attributes, #body)
            })
        }
    }
}
