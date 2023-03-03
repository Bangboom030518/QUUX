use super::{super::GLOBAL_ID, Attributes};
use crate::view::parse::{
    element::{
        children::{Items, ReactiveStore},
        ComponentInitialisationCode,
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
        let super::Data {
            mut component_nodes,
            html,
            mut component_constructors,
        } = item.into();
        self.component_initialisation_code
            .merge(ComponentInitialisationCode {
                nodes: component_nodes,
                constructors: component_constructors,
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

#[derive(Default)]
struct Data {
    tag_name: String,
    attributes: Attributes,
    id: String,
    component_nodes: Vec<TokenStream>,
    component_constructors: Vec<TokenStream>,
    /// The string of html sent to the client
    html: TokenStream,
}

impl From<Element> for Data {
    fn from(
        Element {
            tag_name,
            attributes,
            children,
            ..
        }: Element,
    ) -> Self {
        let mut data = Self {
            tag_name,
            attributes: attributes.into(),
            id: GLOBAL_ID.fetch_add(1, Relaxed).to_string(),
            ..Default::default()
        };
        data.add_children_data(children);
        data.add_attribute_data();
        data
    }
}

impl Element {
    const SELF_CLOSING_ELEMENTS: &'static [&'static str] = &[
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "source", "source",
        "track", "wbr",
    ];

    fn is_self_closing(&self) -> bool {
        Element::SELF_CLOSING_ELEMENTS.contains(&self.tag_name.to_lowercase().as_str())
    }

    /// Generates the html opening and closing tags
    fn html_tag_tokens(&self) -> TokenStream {
        let attributes = TokenStream::from(self.attributes.clone());
        let tag_name = &self.tag_name;
        if self.is_self_closing() {
            quote! {
                format!("<{} {} />", #tag_name, #attributes)
            }
        } else {
            quote! {
                format!("<{0} {1}>{{}}</{0}>", #tag_name, #attributes)
            }
        }
    }

    /// Generates the html body for an element
    fn html_body_tokens(&mut self) -> TokenStream {
        if !matches!(&self.children, Children::Items(items) if children.items.is_empty()) {
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

impl Data {
    fn get_item_html(&mut self, item: Item) -> TokenStream {
        let super::Data {
            mut component_nodes,
            html,
            mut component_constructors,
        } = item.into();
        self.component_nodes.append(&mut component_nodes);
        self.component_constructors
            .append(&mut component_constructors);
        quote! { &#html }
    }

    fn add_attribute_data(&mut self) {
        self.attributes.add_scoped_id(&self.id);

        self.html = self.get_html_tokenstream();
    }
}

impl From<Data> for super::Data {
    fn from(data: Data) -> Self {
        Self {
            component_constructors: data.component_constructors,
            component_nodes: data.component_nodes,
            html: data.html,
        }
    }
}

impl From<Element> for super::Data {
    fn from(element: Element) -> Self {
        Self {
            component_constructors: element,
            component_nodes: data.component_nodes,
            html: element.html_tag_tokens(),
        }
    }
}
