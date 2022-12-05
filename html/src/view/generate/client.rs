use super::super::parse::{Element, Item, Children};
use proc_macro2::{Ident, TokenStream};
use quote::quote;

#[derive(Default)]
struct Data {
    components: Vec<Ident>,
}

impl Data {
    fn new() -> Self {
        Self::default()
    }
}

impl From<Item> for Data {
    fn from(item: Item) -> Data {
        match item {
            Item::Component(component) => Data {
                components: vec![component.name],
            },
            Item::Element(element) => element.into(),
            Item::Expression(_) => Data::new()
        }
    }
}

impl From<Element> for Data {
    fn from(
        Element {
            attributes,
            children,
            ..
        }: Element,
    ) -> Self {
        match children {
            Children::Children(children) => {
                let components = children
                    .into_iter()
                    .flat_map(|node| Self::from(node).components)
                    .collect();
                Self { components }
            },
            Children::ReactiveStore(store) => {
                Self::new()
            }
        }
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let tree = tree.clone();
    let Data { components } = Item::Element(tree).into();
    let components = components.into_iter().map(|ident|  quote! {
        {
            let child = children.next().expect("Client and server child lists don't match");
            let component: #ident = shared::postcard::from_bytes(&child.component).expect("Couldn't deserialize component");
            component.render(child.render_context);
        }
    });
    let tokens = quote! {
        let mut children = context.children.into_iter();
        #( #components  )*
    };
    std::fs::write("expansion.rs", quote! {fn main() {#tokens}}.to_string()).unwrap();
    tokens
}
