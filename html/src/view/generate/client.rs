use super::GLOBAL_ID;
use crate::view::parse::{Attribute, AttributeValue, Children, Element, Item};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use std::sync::atomic::Ordering::Relaxed;
use syn::Expr;

#[derive(Default)]
struct Data {
    components: Vec<Ident>,
    /// Code to update DOM on changes - hydration
    reactivity: Vec<TokenStream>,
    scoped_id: String,
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
                ..Default::default()
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
        let mut data = Self {
            scoped_id: GLOBAL_ID.fetch_add(1, Relaxed).to_string(),
            ..Default::default()
        };
        data.add_event_data(attributes);
        match children {
            Children::Children(children) => data.add_child_data(children),
            Children::ReactiveStore(store) => data.add_store_data(store),
        };
        data
    }
}

impl Data {
    fn add_event_data(&mut self, attributes: Vec<Attribute>) {
        for Attribute { key, value } in attributes {
            if let AttributeValue::Static(value) = value {
                let scoped_id = self.scoped_id.as_str();
                if let Some(event_name) = key.strip_prefix("on:") {
                    self.reactivity.push(quote! {
                        let scope_id = Rc::clone(&scope_id);
                        let closure = wasm_bindgen::prelude::Closure::<dyn FnMut()>::new(#value);
                        log("EVENT LISTENER BLOCK WOZ COORLD");
                        web_sys::window()
                            .expect("Failed to get window (quux internal error)")
                            .document()
                            .expect("Failed to get document (quux internal error)")
                            .query_selector(&format!("[data-quux-scope-id='{}'] [data-quux-scoped-id='{}']", scope_id, #scoped_id))
                            .expect("Failed to get element with scoped id (quux internal error)")
                            .expect("Failed to get element with scoped id (quux internal error)")
                            .add_event_listener_with_callback(#event_name, closure.as_ref().unchecked_ref())
                            .expect("Failed to add event (quux internal error)");
                        closure.forget();
                    })
                }
            }
        }
    }

    fn add_child_data(&mut self, children: Vec<Item>) {
        for child in children {
            let Self {
                mut components,
                mut reactivity,
                ..
            } = child.into();
            self.components.append(&mut components);
            self.reactivity.append(&mut reactivity);
        }
    }

    fn add_store_data(&mut self, store: Expr) {
        // TODO: Consider initializing store only once
        // TODO: Consider initializing the document only once
        let scoped_id = self.scoped_id.as_str();
        self.reactivity.push(quote! {
            let scope_id = Rc::clone(&scope_id);
            shared::Store::on_change(&mut #store, move |_, new| {
                wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(
                    web_sys::window()
                        .expect("Failed to get window (quux internal error)")
                        .document()
                        .expect("Failed to get document (quux internal error)")
                        .query_selector(&format!("[data-quux-scope-id='{}'] [data-quux-scoped-id='{}']", scope_id, #scoped_id))
                        .expect("Failed to get element with scoped id (quux internal error)")
                        .expect("Failed to get element with scoped id (quux internal error)")
                )
                    .expect("`JSCast` from `Element` to `HTMLElement` (quux internal error)")
                    .set_inner_text(&std::string::ToString::to_string(new))
            });
        });
        
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let tree = tree.clone();
    let Data {
        components,
        reactivity,
        ..
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
        use wasm_bindgen::JsCast;
        let mut children = context.children.into_iter();
        let scope_id = Rc::new(context.id);
        #( #components )*
        #({ #reactivity });*
    };
    std::fs::write(
        "expansion-client.rs",
        quote! {fn main() {#tokens}}.to_string(),
    )
    .unwrap();
    tokens
}
