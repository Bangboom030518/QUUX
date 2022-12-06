use super::GLOBAL_ID;
use crate::view::parse::{Children, Element, Item};
use proc_macro2::{Ident, TokenStream};
use quote::{quote, format_ident};
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
                reactivity: Vec::new(),
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
                        let Self {
                            components,
                            reactivity,
                        } = node.into();
                        (components, reactivity)
                    })
                    .unzip();
                let components = components.concat();
                let reactivity = reactivity.concat();
                Self {
                    components,
                    reactivity,
                }
            }
            Children::ReactiveStore(store) => {
                // TODO: Consider initializing store only once
                // TODO: Consider initializing the document only once
                let id = GLOBAL_ID.fetch_add(1, Relaxed);
                let scope_id = format_ident!("scope_id_{}", id);
                Self {
                    components: Vec::new(),
                    reactivity: vec![quote! {
                        let #scope_id = Rc::clone(&scope_id);
                        shared::Store::on_change(&mut #store, move |_, new| {
                            wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(
                                web_sys::window()
                                    .expect("Failed to get window (quux internal error)")
                                    .document()
                                    .expect("Failed to get document (quux internal error)")
                                    .query_selector(&format!("[data-quux-scope-id='{}'] [data-quux-scoped-id='{}']", #scope_id, #id))
                                    .expect("Failed to get element with scoped id (quux internal error)")
                                    .expect("Failed to get element with scoped id (quux internal error)")
                            )
                                .expect("`JSCast` from `Element` to `HTMLElement` (quux internal error)")
                                .set_inner_text(&std::string::ToString::to_string(new))
                        });
                    }],
                }
            }
        }
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let tree = tree.clone();
    let Data {
        components,
        reactivity,
    } = Item::Element(tree).into();
    let components = components.into_iter().map(|ident|  quote! {
        {
            let child = children.next().expect("Client and server child lists don't match");
            let mut component: #ident = shared::postcard::from_bytes(&child.component).expect("Couldn't deserialize component");
            component.render(child.render_context);
        }
    });
    let tokens = quote! {
        use std::rc::Rc;
        let mut children = context.children.into_iter();
        let scope_id = Rc::new(context.id);
        #( #components )*
        #( #reactivity );*
    };
    std::fs::write(
        "expansion-client.rs",
        quote! {fn main() {#tokens}}.to_string(),
    )
    .unwrap();
    tokens
}
