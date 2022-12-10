mod attributes;
mod component;
mod element;

use super::GLOBAL_ID;
use crate::view::parse::{Attribute, AttributeValue, Children, Element, Item};
use attributes::Attributes;
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::atomic::Ordering::Relaxed;
use syn::Expr;

#[derive(Default)]
struct Data {
    /// tokens generating static SSR'd html
    html: TokenStream,
    /// tokens generating a `RenderContext` struct
    component_nodes: Vec<TokenStream>,
    /// the component which must be inserted into the view
    component_constructors: Vec<TokenStream>,
}

impl From<Item> for Data {
    /// Generates data for a single item in a view
    fn from(item: Item) -> Self {
        match item {
            Item::Element(element) => element.into(),
            Item::Component(component) => component.into(),
            Item::Expression(expression) => expression.into(),
        }
    }
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
                let Self {
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

impl From<Expr> for Data {
    fn from(expression: Expr) -> Self {
        Self {
            html: quote! {
                #expression.to_string()
            },
            ..Default::default()
        }
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let mut tree = tree.clone();
    tree.attributes.push(Attribute {
        key: "data-quux-scope-id".to_string(),
        value: AttributeValue::Static(
            syn::parse(quote! { scope_id }.into())
                .expect("Couldn't parse `scope_id` as Expr (quux internal error)"),
        ),
    });
    let Data {
        html,
        component_nodes,
        component_constructors,
    } = Item::Element(tree).into();

    let tokens = quote! {
        let scope_id = shared::generate_id();
        #(#component_constructors)*
        shared::RenderData {
            html: #html,
            component_node: shared::ClientComponentNode {
                component: shared::postcard::to_stdvec(self).expect("Couldn't serialize component (quux internal error)"),
                render_context: shared::RenderContext {
                    id: scope_id,
                    children: vec![
                        #(#component_nodes),*
                    ],
                }
            }
        }
    };
    std::fs::write(
        "expansion-server.rs",
        quote! {fn main() {#tokens}}.to_string(),
    )
    .unwrap();
    tokens
}
