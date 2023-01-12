mod attributes;
mod component;
mod element;

use super::{parse, GLOBAL_ID};
use crate::view::parse::{Attribute, AttributeValue, Element, Item};
use attributes::Attributes;
use proc_macro2::TokenStream;
use quote::quote;
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
        value: AttributeValue::Static(parse(quote! { scope_id })),
    });
    let Data {
        html,
        component_nodes,
        component_constructors,
    } = Item::Element(tree.clone()).into();

    let tokens = quote! {
        let scope_id = context.id;
        #(#component_constructors)*
        shared::RenderData {
            html: #html,
            component_node: shared::ClientComponentNode {
                component: shared::SerializePostcard::serialize_bytes(self),
                render_context: shared::RenderContext {
                    id: scope_id,
                    children: vec![#(#component_nodes),*],
                }
            }
        }
    };
    if let Some(attr) = tree.attributes.first() {
        if attr.key == "magic" {
            std::fs::write(
                "expansion-server.rs",
                quote! {fn main() {#tokens}}.to_string(),
            )
            .unwrap();
        }
    }
    tokens
}
