use super::parse;
use crate::view::parse::prelude::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Expr;

mod attributes;
mod component;
mod element;
mod for_loop;

/// The generation code for an item
#[derive(Clone, Default)]
pub struct Html(pub TokenStream);

impl ToTokens for Html {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // TODO: remove clone?
        tokens.extend(self.0.clone());
    }
}

impl From<Item> for Html {
    fn from(item: Item) -> Self {
        match item {
            Item::Element(element) => element.into(),
            Item::Component(component) => component.into(),
            Item::Expression(expression) => expression.into(),
        }
    }
}

impl From<Expr> for Html {
    fn from(expression: Expr) -> Self {
        Self(quote! {
            #expression.to_string()
        })
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let html = Html::from(tree.clone()).0;

    let tokens = quote! {
        let scope_id = context.id;
        let mut for_loop_children: Vec<Vec<quux::ClientComponentNode<Self::ComponentEnum>>> = Vec::new();
        let mut components = Vec::<quux::ClientComponentNode<Self::ComponentEnum>>::new();

        quux::RenderData {
            html: #html,
            component_node: quux::ClientComponentNode {
                component: Self::ComponentEnum::from(self.clone()),
                render_context: quux::RenderContext {
                    id: scope_id,
                    children: components,
                    for_loop_children,
                }
            }
        }
    };
    // TODO: remove
    if tree.attributes.attributes.contains_key("magic") {
        std::fs::write(
            "expansion-server.rs",
            quote! {fn main() {#tokens}}.to_string(),
        )
        .unwrap();
    }
    tokens
}
