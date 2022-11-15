use std::collections::HashMap;

use super::parse::{Attribute, AttributeValue, Element, Item};
use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Expr, ExprPath, Ident, Path, PathArguments, PathSegment};

#[cfg(target = "wasm")]
pub fn generate(tree: Item) -> TokenStream {
    quote! {
        todo!("Add WASM logic")
    }
    .into()
}

fn ident_to_expr(ident: Ident) -> Expr {
    Expr::Path(ExprPath {
        attrs: Vec::new(),
        qself: None,
        path: Path {
            leading_colon: None,
            segments: {
                let mut punctuated = Punctuated::new();
                punctuated.push(PathSegment {
                    arguments: PathArguments::None,
                    ident,
                });
                punctuated
            },
        },
    })
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
                    (key, ident_to_expr(ident))
                }
            }
        }));
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
            let html_string = format!("<{0} {{}}>{{}}</{0}>", tag_name.to_string());
            quote! {
                format!(#html_string)
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
