use super::parse;
use crate::view::parse::prelude::{element::GenerationData, *};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Expr;

mod attributes;
mod component;
mod element;
mod for_loop;

impl Item {
    // fn component_initialisation_code(&self) -> GenerationData {
    //     match self {
    //         Self::Component(component) => component.clone().into(),
    //         Self::Element(element) => element.component_initialisation_code.clone(),
    //         Self::Expression(_) => Default::default(),
    //     }
    // }
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
        }
    }
}

impl ToTokens for GenerationData {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let html = &self.html;
        tokens.extend(quote! {
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
        });
        // let component_nodes = &value.component_nodes;
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    // let mut tree = tree.clone();
    let render_data = GenerationData::from(tree.clone());

    let tokens = quote! {
        let scope_id = context.id;
        let mut for_loop_children: Vec<Vec<quux::ClientComponentNode<Self::ComponentEnum>>> = Vec::new();
        // #(#component_constructors)*
        // quux::RenderData {
        //     html: #html,
        //     component_node: quux::ClientComponentNode {
        //         component: Self::ComponentEnum::from(self.clone()),
        //         render_context: quux::RenderContext {
        //             id: scope_id,
        //             children: vec![#(#component_nodes),*],
        //             for_loop_children,
        //         }
        //     }
        // }
        #render_data
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
