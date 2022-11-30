#![cfg(not(target = "wasm"))]

use super::super::parse::{Attribute, AttributeValue, Component, Element, Item, Prop};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use shared::generate_id;
use std::collections::HashMap;
use syn::Expr;

struct Attributes {
    keys: Vec<String>,
    values: Vec<Expr>,
    dyn_attributes: HashMap<String, Expr>,
}

impl From<Vec<Attribute>> for Attributes {
    fn from(attributes: Vec<Attribute>) -> Self {
        let mut dyn_attributes = HashMap::new();
        let (keys, values): (Vec<_>, Vec<_>) = attributes
            .into_iter()
            .map(|Attribute { key, value }| {
                let key = key.to_string();
                match value {
                    AttributeValue::Static(expr) => (key, expr),
                    AttributeValue::Reactive(expr) => {
                        dyn_attributes.insert(key.clone(), expr.clone());
                        (
                            key,
                            syn::parse::<Expr>(
                                quote! {
                                    quux::Store::get(#expr)
                                }
                                .into(),
                            )
                            .expect("failed to parse `quux::Store::get(#ident)` (internal)"),
                        )
                    }
                }
            })
            .unzip();
        Self {
            keys,
            values,
            dyn_attributes,
        }
    }
}

#[derive(Default)]
struct Data {
    /// tokens generating static SSR'd html
    html: TokenStream,
    /// tokens generating a `RenderContext` struct
    component_nodes: Vec<TokenStream>,
    /// the component which must be inserted into the view
    component_constructors: Vec<TokenStream>,
}

/// Generates data for a single item in a view
fn read_item(item: Item, data: &Data) -> Data {
    match item {
        Item::Element(Element {
            tag_name,
            attributes,
            content,
        }) => {
            // TODO: deal with reactive stores as attribute values
            // TODO: make WORK
            let Attributes {
                keys,
                values,
                dyn_attributes,
            } = attributes.into();
            let html_string = format!(
                "<{0} {1}>{{}}</{0}>",
                tag_name.to_string(),
                keys.into_iter()
                    .map(|key| format!("{key}=\"{{}}\""))
                    .collect::<String>(),
            );
            let (mut html, (component_nodes, component_constructors)): (Vec<_>, (Vec<_>, Vec<_>)) =
                content
                    .into_iter()
                    .map(|item| {
                        let Data {
                            component_nodes,
                            html,
                            component_constructors,
                        } = read_item(item, data);
                        (quote! { &#html }, (component_nodes, component_constructors))
                    })
                    .unzip();

            let component_nodes = component_nodes.concat();
            let component_constructors = component_constructors.concat();

            html.insert(0, quote! { String::new() });

            let html = if values.is_empty() {
                quote! {
                    format!(#html_string, #(#html)+*)
                }
            } else {
                quote! {
                    format!(#html_string, #(#values),*, #(#html)+*)
                }
            };

            Data {
                html,
                component_nodes,
                component_constructors,
            }
        }
        Item::Component(Component { name, props }) => {
            let component_id = generate_id();
            let component_ident = format_ident!("component_{}", component_id);
            let rendered_component_ident = format_ident!("rendered_component_{}", component_id);
            let props = props.into_iter().map(|Prop { key, value }| {
                quote! { #key : #value }
            });
            let component_nodes = vec![quote! {
                shared::ClientComponentNode {
                    component: Box::new(#component_ident),
                    render_context: shared::RenderContext {
                        id: shared::generate_id(),
                        children: Vec::new(),
                    },
                    static_id: #component_id,
                }
            }];
            let component_constructors = vec![quote! {
                let #component_ident = #name ::init(<#name as shared::Component>::Props {
                    #(#props),*
                });
                let #rendered_component_ident = #component_ident.render();
            }];

            Data {
                html: quote! { #rendered_component_ident.html },
                component_nodes,
                component_constructors,
            }
        }
        Item::Expression(expression) => Data {
            html: quote! {
                #expression.to_string()
            },
            component_nodes: Vec::new(),
            component_constructors: Vec::new(),
        },
        Item::ReactiveStore(_) => todo!("Implement Reactive Stores"),
    }
}

#[cfg(not(target = "wasm"))]
pub fn generate(tree: Element) -> TokenStream {
    let Data {
        html,
        component_nodes,
        component_constructors,
    } = read_item(Item::Element(tree), &Data::default());
    let tokens = quote! {
        {
            #(#component_constructors)*
            shared::RenderData {
                html: #html,
                render_context: shared::RenderContext {
                    id: shared::generate_id(),
                    children: vec![
                        #(#component_nodes),*
                    ],
                }
            }
        }
    };
    std::fs::write("expansion.rs", quote! {fn main() {#tokens}}.to_string()).unwrap();
    tokens
}
