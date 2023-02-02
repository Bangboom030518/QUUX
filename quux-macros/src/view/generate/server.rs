use super::parse;
use crate::view::parse::prelude::{element::Attribute, *};
use attributes::Attributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

mod attributes;
mod component;
mod element;

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
    // let mut tree = tree.clone();
    // tree.attributes.push(Attribute {
    //     key: "data-quux-scope-id".to_string(),
    //     value: AttributeValue::Static(parse(quote! { scope_id })),
    // });
    let Data {
        html,
        component_nodes,
        component_constructors,
    } = tree.clone().into();

    // TODO: remove
    if let Some(Attribute { key, .. }) = tree.attributes.first() {
        if key == "magic" {
            std::fs::write(
                "nuclear-waste-facility.txt",
                format!(
                    "{}\n\n\n{}",
                    component_nodes
                        .iter()
                        .map(ToString::to_string)
                        .intersperse("\n".to_string())
                        .collect::<String>(),
                    component_constructors
                        .iter()
                        .map(ToString::to_string)
                        .intersperse("\n".to_string())
                        .collect::<String>(),
                ),
            )
            .unwrap();
        }
    }

    let tokens = quote! {
        let scope_id = context.id;
        let mut dynamic_component_nodes: Vec<quux::ClientComponentNode<T>> = Vec::new();
        #(#component_constructors)*
        quux::RenderData {
            html: #html,
            component_node: quux::ClientComponentNode {
                component: quux::SerializePostcard::serialize_bytes(self),
                render_context: quux::RenderContext {
                    id: scope_id,
                    children: vec![vec![#(#component_nodes),*], dynamic_component_nodes].concat(),
                }
            }
        }
    };
    // TODO: remove
    if let Some(attribute) = tree.attributes.first() {
        if attribute.key == "magic" {
            std::fs::write(
                "expansion-server.rs",
                quote! {fn main() {#tokens}}.to_string(),
            )
            .unwrap();
        }
    }
    tokens
}
