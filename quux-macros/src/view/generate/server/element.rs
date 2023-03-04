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
use quote::quote;
use std::{collections::VecDeque, sync::atomic::Ordering::Relaxed};

impl From<ReactiveStore> for TokenStream {
    /// Generates the body of an element.
    fn from(ReactiveStore(store): ReactiveStore) -> Self {
        quote! { #store.get() }
    }
}

impl Items {
    /// Mutates the initialisation code with the initialisation required by the child elements.
    fn item_html(&mut self, item: Item) -> TokenStream {
        let GenerationData {
            mut component_nodes,
            html,
            mut component_constructors,
        } = item.into();
        self.component_initialisation_code.merge(GenerationData {
            component_nodes,
            component_constructors,
            ..Default::default()
        });
        quote! { &#html }
    }

    /// Generates the body of an element.
    /// Mutates the initialisation code with the initialisation required by the child elements.
    pub fn html_body_tokens(&mut self) -> TokenStream {
        let mut html: VecDeque<_> = self
            .items
            .into_iter()
            .map(|item| self.item_html(item))
            .collect();
        html.push_back(quote! { String::new() });
        let html = html.into_iter();
        quote! {
            #(#html)+*
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
    /// Muatates the element to include any component initialisation logic needed by the body.
    fn html_body_tokens(&mut self) -> TokenStream {
        if !matches!(&self.children, Children::Items(items) if items.items.is_empty()) {
            assert!(
                !self.is_self_closing(),
                "Self-closing elements cannot contain children"
            );
        }
        match self.children {
            Children::Items(items) => {
                let tokens = items.html_body_tokens();
                self.component_initialisation_code
                    .merge(items.component_initialisation_code);
                tokens
            }
            Children::ReactiveStore(store) => {
                self.attributes.element_needs_id = true;
                store.into()
            }
            Children::ForLoop(for_loop) => for_loop.into(),
        }
    }
}

impl From<Element> for GenerationData {
    fn from(value: Element) -> Self {
        let attributes = TokenStream::from(value.attributes.clone());
        let tag_name = &value.tag_name;
        if value.is_self_closing() {
            Self {
                html: quote! {
                    format!("<{} {} />", #tag_name, #attributes)
                },
                ..value.component_initialisation_code
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
                ..value.component_initialisation_code
            }
        }
    }
}
