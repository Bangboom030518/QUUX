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
    /// tokens generating static SSR'd html
    html: TokenStream,
    /// tokens generating a `RenderContext` struct
    render_context: TokenStream,
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
            let (attributes, dyn_attributes) = format_attributes(attributes);
            let render_context = quote! {};
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
                        render_context,
                        html,
                    } = generate_render_data(item);
                    RenderData {
                        html: quote! { &#html },
                        render_context: quote! { #render_context }
                }})
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
            RenderData {
                html,
                render_context,
            }
        },
        Item::Component(component) => RenderData {
            html: quote! { String::new() },
            render_context: quote! {  },
        },
        Item::Expression(expression) => RenderData {
            html: quote! {
                #expression.to_string()
            },
            render_context: quote! {  },
        },
        Item::ReactiveStore(_) => todo!("Implement Reactive Stores"),
    }
}

#[cfg(not(target = "wasm"))]
pub fn generate(tree: Item) -> TokenStream {
    let RenderData {
        html,
        render_context,
    } = generate_render_data(tree);
    quote! {
        shared::RenderData {
            html: #html,
            render_data: #ids
        }
    }
}
