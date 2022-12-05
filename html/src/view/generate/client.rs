use super::GLOBAL_ID;
use crate::view::parse::{Children, Element, Item};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Default)]
struct Data {
    components: Vec<Ident>,
    /// Code to update DOM on changes - hydration
    reactivity: Vec<TokenStream>,
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
                reactivity: Vec::new()
            },
            Item::Element(element) => element.into(),
            Item::Expression(_) => Data::new(),
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
                let (components, reactivity): (Vec<_>, Vec<_>) = children
                    .into_iter()
                    .map(|node| {
                        let Self { components, reactivity } = node.into();
                        (components, reactivity)
                    })
                    .unzip();
                let components = components.concat();
                let reactivity = reactivity.concat();
                Self { components, reactivity }
            }
            Children::ReactiveStore(store) => {
                let id = GLOBAL_ID.fetch_add(1, Relaxed);
                Self {
                    components: Vec::new(),
                    reactivity: vec![
                        // web_sys::window().unwrap().document().unwrap()
                        quote! {
                            store.on_change(|_| log("something changed!"))
                        }
                    ]
                }
            }
        }
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let tree = tree.clone();
    let Data { components, reactivity } = Item::Element(tree).into();
    let components = components.into_iter().map(|ident|  quote! {
        {
            let child = children.next().expect("Client and server child lists don't match");
            let mut component: #ident = shared::postcard::from_bytes(&child.component).expect("Couldn't deserialize component");
            component.render(child.render_context);
        }
    });
    let tokens = quote! {
        let mut children = context.children.into_iter();
        #( #components )*
        #( #reactivity );*
    };
    std::fs::write("expansion.rs", quote! {fn main() {#tokens}}.to_string()).unwrap();
    tokens
}
