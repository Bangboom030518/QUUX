use super::super::parse::{Element, Item};
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

fn read_item(item: Item) -> Data {
    match item {
        Item::Component(component) => Data {
            components: vec![component.name],
        },
        Item::Element(Element {
            tag_name,
            attributes,
            content,
        }) => {
            let components = content
                .into_iter()
                .flat_map(|node| read_item(node).components)
                .collect();
            Data { components }
        }
        Item::Expression(_) => Data::new(),
        Item::ReactiveStore(store) => Data::new(),
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let tree = tree.clone();
    let Data { components } = read_item(Item::Element(tree));
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
