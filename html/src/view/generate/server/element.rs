use std::sync::atomic::Ordering::Relaxed;

use super::{Attributes, GLOBAL_ID};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

use crate::view::parse::{Children, Element, Item};

#[derive(Default)]
struct Data {
    tag_name: String,
    attributes: Attributes,
    id: String,
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
    fn add_children_data(&mut self, children: Children) {
        match children {
            Children::Children(children) => self.add_regular_children_data(children),
            Children::ReactiveStore(store) => self.add_store_data(&store),
        };
    }

    fn add_item_data(&mut self, item: Item) -> TokenStream {
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

    fn add_regular_children_data(&mut self, children: Vec<Item>) {
        let mut html: Vec<_> = children
            .into_iter()
            .map(|item| self.add_item_data(item))
            .collect();
        html.insert(0, quote! { String::new() });
        self.html = quote!(#(#html)+*);
    }

    fn add_attribute_data(&mut self) {
        self.attributes.add_scoped_id(&self.id);
        let html_string = self.get_html_string();
        let html = &self.html;
        let values = &self.attributes.values;

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

    fn get_html_string(&self) -> String {
        let attributes = self
            .attributes
            .keys
            .iter()
            .map(|key| format!("{key}=\"{{}}\""))
            .collect::<String>();
        format!("<{0} {1}>{{}}</{0}>", self.tag_name, attributes)
    }

    fn add_store_data(&mut self, store: &Expr) {
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
