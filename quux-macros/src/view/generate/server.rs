use super::parse;
use crate::view::parse::prelude::{
    element::{Attributes, GenerationData},
    *,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

mod attributes;
mod component;
mod element;
mod for_loop;

impl Item {
    fn component_initialisation_code(&self) -> GenerationData {
        match self {
            Self::Component(component) => todo!(),
            Self::Element(element) => element.component_initialisation_code,
            Self::Expression(_) => Default::default(),
        }
    }
}

// #[derive(Default)]
// struct Data {
//     /// tokens generating static SSR'd html
//     html: TokenStream,
//     component_initialisation_code: GenerationData,
// }

// impl Data {
//     fn from_for_loop_inner(item: Item) -> Self {
//         match item {
//             Item::Element(element) => todo!(),
//             Item::Component(component) => todo!(),
//             Item::Expression(expression) => {
//                 panic!("Reactive for loops must contain elements or components. Found expression.")
//             }
//         }
//     }
// }

impl From<Item> for GenerationData {
    fn from(item: Item) -> Self {
        match item {
            Item::Element(element) => element.into(),
            Item::Component(component) => component.into(),
            Item::Expression(expression) => expression.into(),
        }
    }
}

impl From<Expr> for GenerationData {
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
    let GenerationData {
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
        let mut for_loop_children: Vec<Vec<quux::ClientComponentNode<Self::ComponentEnum>>> = Vec::new();
        #(#component_constructors)*
        quux::RenderData {
            html: #html,
            component_node: quux::ClientComponentNode {
                component: Self::ComponentEnum::from(self.clone()),
                render_context: quux::RenderContext {
                    id: scope_id,
                    children: vec![#(#component_nodes),*],
                    for_loop_children,
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
