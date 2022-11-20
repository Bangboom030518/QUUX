use std::collections::HashMap;

use super::parse::{Attribute, AttributeValue, Element, Item};
// use proc_macro::TokenStream;
use quote::{quote, ToTokens, __private::TokenStream};
use std::convert::Into;
use syn::Expr;

#[cfg(target = "wasm")]
pub fn generate(tree: Item) -> TokenStream {
    quote! {
        todo!("Add WASM logic")
    }
    .into()
}

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

struct RenderData {
    html: TokenStream,
    /// A Vec of entries to a HashMap
    ids: Vec<TokenStream>,
}

fn generate_render_data(item: Item) -> RenderData {
    match item {
        Item::Element(Element {
            tag_name,
            attributes,
            content,
        }) => {
            // TODO: deal with reactive stores as attribute values
            // TODO: make WORK
            let mut ids = HashMap::new();
            let (attributes, dyn_attributes) = format_attributes(attributes);
            let (keys, values): (Vec<_>, Vec<_>) = attributes.into_iter().unzip();
            let html_string = format!(
                "<{0} {1}>{{}}</{0}>",
                tag_name.to_string(),
                keys.into_iter()
                    .map(|key| format!("{key}=\"{{}}\""))
                    .collect::<String>(),
            );
            let mut content = content
                .into_iter()
                .map(|item| {
                    let RenderData {
                        ids: item_ids,
                        html,
                    } = generate_render_data(item);
                    ids.extend(item_ids.into_iter());
                    quote! { &#html }
                })
                .collect::<Vec<_>>();
            content.insert(0, quote! { String::new() });
            let html = if values.is_empty() {
                quote! {
                    format!(#html_string, #(#content)+*)
                }
            } else {
                quote! {
                    format!(#html_string, #(#values),*, #(#content)+*)
                }
            };
            RenderData { html, ids: format_hashmap(ids) }
        }
        Item::Component(component) => {
            RenderData {
                html: quote!{ String::new() },
                ids: format_hashmap(HashMap::new()),
            }
        },
        Item::Expression(expression) => RenderData {
            html: quote! {
                #expression.to_string()
            },
            ids: format_hashmap(HashMap::new()),
        },
        Item::ReactiveStore(_) => todo!("Implement Reactive Stores"),
    }
}

fn format_hashmap<K, V>(hashmap: HashMap<K, V>) -> TokenStream
where
    K: ToTokens,
    V: ToTokens,
{
    let entries = hashmap.into_iter().map(|(key, value)| {
        quote! {
            (#key, #value)
        }
    });

    quote! {
        std::collections::HashMap::from([#(#entries),*])
    }
}

#[cfg(not(target = "wasm"))]
pub fn generate(tree: Item) -> TokenStream {
    let RenderData { html, ids } = generate_render_data(tree);
    quote! {
        shared::RenderData {
            html: #html,
            ids: #ids
        }
    }
}
