use super::super::parse::{Attribute, AttributeValue, Element, Item, Component, Prop};
use proc_macro2::TokenStream;
use quote::quote;
use shared::generate_id;
use std::collections::HashMap;
use syn::Expr;


fn format_attributes(
    attributes: Vec<Attribute>,
) -> (HashMap<String, Expr>, HashMap<String, String>) {
    let mut dyn_attributes: HashMap<String, String> = HashMap::new();
    let attributes: HashMap<String, Expr> =
        HashMap::from_iter(attributes.into_iter().map(|Attribute { key, value }| {
            let key = key.to_string();
            match value {
                AttributeValue::Static(expr) => (key, *expr),
                AttributeValue::Reactive(ident) => {
                    dyn_attributes.insert(key.clone(), ident.to_string());
                    (
                        key,
                        syn::parse::<Expr>(
                            quote! {
                                quux::Store::get(#ident)
                            }
                            .into(),
                        )
                        .expect("failed to parse `quux::Store::get(#ident)` (internal)"),
                    )
                }
            }
        }));
    (attributes, dyn_attributes)
}

#[derive(Default)]
struct Data {
    /// tokens generating static SSR'd html
    html: TokenStream,
    /// tokens generating a `RenderContext` struct
    component_nodes: Vec<TokenStream>,
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
            let (attributes, dyn_attributes) = format_attributes(attributes);
            let (keys, values): (Vec<_>, Vec<_>) = attributes.into_iter().unzip();
            let html_string = format!(
                "<{0} {1}>{{}}</{0}>",
                tag_name.to_string(),
                keys.into_iter()
                    .map(|key| format!("{key}=\"{{}}\""))
                    .collect::<String>(),
            );
            let (mut html, component_nodes): (Vec<_>, Vec<_>) = content
                .into_iter()
                .map(|item| {
                    let Data {
                        component_nodes,
                        html,
                    } = read_item(item, data);
                    (quote! { &#html }, component_nodes)
                })
                .unzip();
            
            let component_nodes = component_nodes.concat();

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
            }
        }
        Item::Component(Component { name, props }) => {
            let props = props.into_iter().map(|Prop { key, value }| {
                quote!{ #key : #value }
            });
            let mut component_nodes = data.component_nodes.clone();
            let id = generate_id();
            component_nodes.push(quote! {
                shared::ClientComponentNode {
                    component: &#name ::init( <#name as shared::Component>::Props {
                        #(#props),*
                    }),
                    render_context: shared::RenderContext {
                        id: shared::generate_id(),
                        children: Vec::new(),
                    },
                    static_id: #id,
                }
            });
            Data {
                html: quote! { String::new() },
                component_nodes,
            }
        }
        Item::Expression(expression) => Data {
            html: quote! {
                #expression.to_string()
            },
            component_nodes: data.component_nodes.clone(),
        },
        Item::ReactiveStore(_) => todo!("Implement Reactive Stores"),
    }
}

#[cfg(not(target = "wasm"))]
pub fn generate(tree: Element) -> TokenStream {
    let Data { html, component_nodes } = read_item(Item::Element(tree), &Data::default());
    quote! {
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
}
