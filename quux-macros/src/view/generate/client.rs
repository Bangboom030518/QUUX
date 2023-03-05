use super::GLOBAL_ID;
use crate::view::parse::prelude::*;
use element::{children::Items, children::ReactiveStore, Attributes, Children};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::sync::atomic::Ordering::Relaxed;
use syn::Expr;

mod for_loop;

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
                            #binding = component.try_into().unwrap()
                        }
                    });

                    let component_string = component.name.to_token_stream().to_string();
                    vec![quote! {
                        {
                            let child = children.next().expect_internal(concat!("retrieve all child data (", #component_string, ") : client and server child lists don't match"));
                            let mut component = child.component;
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
            Children::Items(children) => data.add_child_data(children),
            Children::ReactiveStore(store) => data.add_store_data(&store),
            Children::ForLoop(for_loop) => {
                data.reactivity.push(for_loop.reactivity_code());
            }
        };
        data
    }
}

impl Data {
    fn add_attribute_data(&mut self, key: String, value: Expr) {
        // let attribute::Value::Static(value) = value else {
        //     return
        // };

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

    fn add_event_data(&mut self, attributes: Attributes) {
        for (key, value) in attributes.attributes {
            self.add_attribute_data(key, value);
        }
    }

    fn add_child_data(&mut self, children: Items) {
        for child in children.items {
            let Self {
                mut components,
                mut reactivity,
                ..
            } = child.into();
            self.components.append(&mut components);
            self.reactivity.append(&mut reactivity);
        }
    }

    fn add_store_data(&mut self, ReactiveStore(store): &ReactiveStore) {
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
    let tokens = quote! {
        use std::rc::Rc;
        use wasm_bindgen::JsCast;
        use quux::errors::MapInternal;
        let mut children = context.children.into_iter();
        let mut for_loop_children = context.for_loop_children.into_iter();
        let scope_id = Rc::new(context.id);
        #(#components);*;
        // TODO: what about non-for-loop components?
        for child in children {
            let mut component = child.component;
            component.render(child.render_context);
        }
        #({ #reactivity });*;
        quux::RenderData::new()
    };
    if tree.attributes.attributes.contains_key("magic") {
        std::fs::write(
            "expansion-client.rs",
            quote! {fn main() {#tokens}}.to_string(),
        )
        .unwrap();
    }
    tokens
}
