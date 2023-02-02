use super::GLOBAL_ID;
use crate::view::parse::prelude::*;
use element::{attribute, Attribute, Children};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::sync::atomic::Ordering::Relaxed;
use syn::Expr;

#[derive(Default)]
struct Data {
    components: Vec<TokenStream>,
    /// Code to update DOM on changes - hydration
    reactivity: Vec<TokenStream>,
    id: String,
}

impl Data {
    fn new() -> Self {
        Self::default()
    }
}

impl From<Item> for Data {
    fn from(item: Item) -> Self {
        match item {
            Item::Component(component) => Self {
                components: {
                    let binding = component.binding.map_or_else(TokenStream::new, |binding| {
                        quote! {
                            #binding = component
                        }
                    });

                    let component_string = component.name.to_token_stream().to_string();
                    let name = component.name;

                    vec![quote! {
                        {
                            let child = children.next().expect_internal(concat!("retrieve all child data (", #component_string, ") : client and server child lists don't match"));
                            let mut component: #name = quux::postcard::from_bytes(&child.component).expect("Couldn't deserialize component");
                            component.render(child.render_context);
                            #binding;
                        }
                    }]
                },
                reactivity: Vec::new(),
                ..Default::default()
            },
            Item::Element(element) => element.into(),
            Item::Expression(_) => Self::new(),
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
            id: GLOBAL_ID.fetch_add(1, Relaxed).to_string(),
            ..Default::default()
        };
        data.add_event_data(attributes);
        match children {
            Children::Children(children) => data.add_child_data(children),
            Children::ReactiveStore(store) => data.add_store_data(&store),
            Children::ForLoop(_) => {} // TODO: reactive for????
        };
        data
    }
}

impl Data {
    fn add_attribute_data(&mut self, Attribute { key, value }: Attribute) {
        let attribute::Value::Static(value) = value else {
            return
        };

        if let Some(event_name) = key.strip_prefix("on:") {
            let scoped_id = self.id.as_str();

            self.reactivity.push(quote! {
                let scope_id = Rc::clone(&scope_id);
                let closure = wasm_bindgen::prelude::Closure::<dyn FnMut()>::new(#value);
                quux::dom::get_reactive_element(&*scope_id, #scoped_id)
                    .add_event_listener_with_callback(#event_name, closure.as_ref().unchecked_ref())
                    .expect_internal("add event");
                closure.forget();
            });
        } else if key == "class:active-when" {
            let scoped_id = self.id.as_str();

            self.reactivity.push(quote! {
                let (store, mapping, class_name) = #value;
                let store = quux::Store::clone(store);
                let scope_id = Rc::clone(&scope_id);
                let class_list = quux::dom::get_reactive_element(&*scope_id, #scoped_id).class_list();
                store.on_change(move |previous, current| if mapping(std::clone::Clone::clone(current)) {
                    class_list.add_1(class_name).unwrap();
                } else {
                    class_list.remove_1(class_name).unwrap();
                })
            });
        }
    }

    fn add_event_data(&mut self, attributes: Vec<Attribute>) {
        for attribute in attributes {
            self.add_attribute_data(attribute);
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

    fn add_store_data(&mut self, store: &Expr) {
        // TODO: Consider initializing store only once
        // TODO: Consider initializing the document only once
        let scoped_id = self.id.as_str();
        self.reactivity.push(quote! {
            let scope_id = Rc::clone(&scope_id);
            #store.on_change(move |_, new| {
                let element = quux::dom::get_reactive_element(&*scope_id, #scoped_id);
                quux::dom::as_html_element(element)
                    .set_inner_text(&std::string::ToString::to_string(new));
            });
        });
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let tree = tree.clone();

    // TODO: remove
    // std::fs::write("id.log", "").unwrap();

    let Data {
        components,
        reactivity,
        ..
    } = tree.clone().into();

    // TODO: remove
    let debug_code = if let Some(Attribute { key, .. }) = tree.attributes.first() {
        if key == "magic" {
            quote! {
                // panic!("{:?}", children.map(|child| format!("{:?}", child)).collect::<Vec<_>>())
            }
        } else {
            TokenStream::new()
        }
    } else {
        TokenStream::new()
    };
    let tokens = quote! {
        use std::rc::Rc;
        use wasm_bindgen::JsCast;
        use quux::errors::MapInternal;
        let mut children = context.children.into_iter();
        let scope_id = Rc::new(context.id);
        #debug_code;
        #(#components);*;
        #({ #reactivity });*;
        quux::RenderData::new()
    };
    if let Some(Attribute { key, .. }) = tree.attributes.first() {
        if key == "magic" {
            std::fs::write(
                "expansion-client.rs",
                quote! {fn main() {#tokens}}.to_string(),
            )
            .unwrap();
        }
    }
    tokens
}
