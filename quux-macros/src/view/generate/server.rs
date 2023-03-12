use crate::view::parse::prelude::*;
use lazy_static::lazy_static;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_quote, Expr};

#[derive(Clone, Copy)]
struct ConstIdent(&'static str);

impl ToTokens for ConstIdent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        format_ident!("{}", self.0).to_tokens(tokens);
    }
}

lazy_static! {
    static ref ID: ConstIdent = ConstIdent("id");
}

mod attributes;
mod component;
mod element;
mod for_loop;

#[derive(Clone, Default)]
pub struct Html(pub TokenStream);

impl ToTokens for Html {
    fn to_tokens(&self, tokens: &mut TokenStream) {
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

impl Item {
    fn insert_for_loop_id(&mut self, id: u64) {
        let value = parse_quote! {
            format!("{}.{}.{}", context.id, #id, index)
        };
        let unique = match self {
            Self::Element(element) => element
                .insert_attribute("data-quux-for-id", value)
                .is_none(),
            Self::Component(component) => component.insert_for_loop_id(value).is_none(),
            Self::Expression(_) => {
                panic!("Reactive for loops must contain either elements or components. Found expression")
            }
        };
        assert!(unique, "duplicate \"data-quux-for-id\" attribute");
    }
}

impl From<Expr> for Html {
    fn from(expression: Expr) -> Self {
        Self(quote! {
            #expression.to_string()
        })
    }
}

pub fn generate(tree: &View) -> TokenStream {
    let View {
        context,
        mut element,
    } = tree.clone();
    element.attributes.is_root = true;
    let Html(html) = Html::from(element.clone());

    let id = *ID;
    let tokens = quote! {
        let context = #context;
        let #id = context.id;
        let mut component_id = context.id;
        let mut for_loop_children: Vec<Vec<quux::render::ClientComponentNode<Self::ComponentEnum>>> = Vec::new();
        let mut components = Vec::<quux::render::ClientComponentNode<Self::ComponentEnum>>::new();
        let for_loop_id = context.for_loop_id;

        quux::render::Output {
            html: #html,
            component_node: quux::render::ClientComponentNode {
                component: Self::ComponentEnum::from(self.clone()),
                render_context: quux::render::Context {
                    id: #id,
                    children: components,
                    for_loop_id: None,
                    for_loop_children,
                }
            }
        }
    };
    if element.attributes.attributes.contains_key("magic") {
        std::fs::write(
            "expansion-server.rs",
            quote! {fn main() {#tokens}}.to_string(),
        )
        .unwrap();
    }
    tokens
}
