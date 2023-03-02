use super::{super::GLOBAL_ID, Attributes};
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::Ordering::Relaxed;
use syn::Expr;

use crate::view::parse::{element::children::ForLoopIterable, prelude::*};
use element::{Children, ForLoop};

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

impl Data {
    const SELF_CLOSING_ELEMENTS: &'static [&'static str] = &[
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "source", "source",
        "track", "wbr",
    ];

    fn is_self_closing(&self) -> bool {
        Self::SELF_CLOSING_ELEMENTS.contains(&self.tag_name.to_lowercase().as_str())
    }

    fn add_children_data(&mut self, children: Children) {
        match children {
            Children::Items(children) => self.add_element_children_data(children),
            Children::ReactiveStore(store) => self.add_store_data(&store),
            Children::ForLoop(for_loop) => self.add_for_loop_data(for_loop),
        };
    }

    fn add_for_loop_data(
        &mut self,
        ForLoop {
            pattern,
            iterable,
            item,
        }: ForLoop,
    ) {
        // TODO: components!!!
        let reactive: bool;
        let super::Data {
            component_nodes,
            html,
            component_constructors,
        } = (*item).into();
        let iterable = match iterable {
            ForLoopIterable::Static(iterable) => {
                reactive = false;
                quote! {
                    #iterable
                }
            }
            ForLoopIterable::Reactive(iterable) => {
                reactive = true;
                quote! {
                    (std::cell::Ref::<Vec<_>>::from(&#iterable)).iter().cloned()
                }
            }
        };
        let id_addition_code = if reactive {
            quote! {
                todo!()
            }
        } else {
            TokenStream::new()
        };
        self.html = quote! {{
            let mut currrent_component_nodes: Vec<_> = Vec::new();
            let html = (#iterable).map(|#pattern| {
                #(#component_constructors);*;
                #(currrent_component_nodes.push(#component_nodes.clone()));*;
                #id_addition_code
                String::from(#html)
            }).collect::<String>();
            for_loop_children.push(currrent_component_nodes);
            html
        }};
    }

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

    fn add_element_children_data(&mut self, children: Vec<Item>) {
        if self.is_self_closing() {
            assert!(
                children.is_empty(),
                "Self-closing element '{}' cannot have children",
                self.tag_name
            );
        }
        let mut html: Vec<_> = children
            .into_iter()
            .map(|item| self.get_item_html(item))
            .collect();
        html.insert(0, quote! { String::new() });
        self.html = quote! {
            #(#html)+*
        };
    }

    // TODO: move to attribute
    fn get_attribute_tokens(&self, Attributes { keys, values, .. }: &Attributes) -> TokenStream {
        let attributes = keys.iter().zip(values).map(|(key, value)| {
            quote! {
                format!("{}=\"{}\"", #key, #value)
            }
        });
        quote! {
            String::new() + #(&#attributes)+*
        }
    }

    fn add_attribute_data(&mut self) {
        self.attributes.add_scoped_id(&self.id);
        // let html_string = self.get_html_tokenstream();
        // let html = &self.html;
        // let values = &self.attributes.values;

        self.html = self.get_html_tokenstream();
    }

    fn get_html_tokenstream(&self) -> TokenStream {
        let attributes = self.get_attribute_tokens(&self.attributes);
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

    fn add_store_data(&mut self, store: &Expr) {
        assert!(
            self.is_self_closing(),
            "Self closing element {} cannot have store children",
            self.tag_name
        );

        self.attributes.reactive = true;
        self.html = quote! { #store.get() };
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
        Data::from(element).into()
    }
}
