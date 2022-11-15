use std::collections::HashMap;

use super::parse::{Attribute, AttributeValue, Element, Item};
use proc_macro::TokenStream;
use quote::quote;
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
    let attributes: HashMap<String, Expr> = HashMap::from_iter(attributes.into_iter().map(
        |Attribute { key, value }| -> (_, _) {
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
        },
    ));
    (attributes, dyn_attributes)
}

#[cfg(not(target = "wasm"))]
pub fn generate(tree: Item) -> TokenStream {
    match tree {
        Item::Element(Element {
            tag_name,
            attributes,
            content,
        }) => {
            // TODO: deal with reactive stores as attribute values
            // TODO; make WORK
            let (attributes, dyn_attributes) = format_attributes(attributes);
            let (keys, values): (Vec<_>, Vec<_>) = attributes.into_iter().unzip();
            let html_string = format!(
                "<{0} {1}>{{}}</{0}>",
                tag_name.to_string(),
                keys.into_iter()
                    .map(|key| format!("{key}=\"{{}}\""))
                    .collect::<String>()
            );
            quote! {
                format!(#html_string, )
            }
        }
        Item::Component(_) => todo!("Implement Component"),
        Item::Expression(_) => todo!("Implement Expressions"),
        Item::ReactiveStore(_) => todo!("Implement Reactive Stores"),
    };

    quote! {
        shared::RenderData {
            html: String
        }
        // todo!("Add server logic")
    }
    .into()
}
