use std::sync::atomic::Ordering::Relaxed;

use super::{Attributes, GLOBAL_ID};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

use crate::view::parse::{Children, Element, Item};

#[derive(Default)]
struct Data {
    component_nodes: Vec<TokenStream>,
    component_constructors: Vec<TokenStream>,
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
        let id = GLOBAL_ID.fetch_add(1, Relaxed).to_string();

        let mut data = Self::default();

        // TODO: deal with reactive stores as attribute values
        let mut attributes: Attributes = attributes.into();

        match children {
            Children::Children(children) => data.add_child_data(children),
            Children::ReactiveStore(store) => {
                attributes.reactive = true;
                data.add_store_data(&store);
            }
        };

        data.add_attribute_data(attributes, &tag_name.to_string(), &id);

        data
    }
}

impl Data {
    fn add_child_data(&mut self, children: Vec<Item>) {
        let mut html: Vec<_> = children
            .into_iter()
            .map(|child| {
                let super::Data {
                    mut component_nodes,
                    html,
                    mut component_constructors,
                } = child.into();
                self.component_nodes.append(&mut component_nodes);

                self.component_constructors
                    .append(&mut component_constructors);
                quote! { &#html }
            })
            .collect();
        html.insert(0, quote! { String::new() });
        self.html = quote!(#(#html)+*);
    }

    fn add_attribute_data(&mut self, mut attributes: Attributes, tag_name: &str, id: &str) {
        attributes.add_scoped_id(id);

        let html_string = Self::get_html_string(attributes.keys, tag_name);
        let html = &self.html;
        let values = attributes.values;
        
        self.html = if values.is_empty() {
            quote! {
                format!(#html_string, #html)
            }
        } else {
            quote! {
                format!(#html_string, #(#values),*, #html)
            }
        };
    }

    fn get_html_string(keys: Vec<String>, tag_name: &str) -> String {
        format!(
            "<{0} {1}>{{}}</{0}>",
            tag_name,
            keys.into_iter()
                .map(|key| format!("{key}=\"{{}}\""))
                .collect::<String>(),
        )
    }

    fn add_store_data(&mut self, store: &Expr) {
        self.html = quote! { shared::Store::get(&#store) };
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
