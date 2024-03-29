use super::internal::prelude::*;
use crate::view::parse::prelude::*;

mod for_loop;

#[derive(Default)]
struct Data {
    components: Vec<Component>,
    /// Code to update DOM on changes - hydration
    reactivity: Vec<TokenStream>,
    for_loops: Vec<ForLoop>,
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
                components: vec![component],
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
        let mut data = Self::default();
        data.add_event_data(attributes.clone());
        match children {
            Children::Items(children) => data.add_child_data(children),
            Children::ReactiveStore(store) => data.add_store_data(&store, attributes.id),
            Children::ForLoop(for_loop) => {
                data.for_loops.push(for_loop);
                // data.reactivity.push(for_loop.reactivity(attributes.id));
            }
            _ => {
                // TODO: handle if and match
            }
        };
        data
    }
}

impl Data {
    fn add_event_data(&mut self, attributes: Attributes) {
        let id = attributes.id;

        for (event, callback) in attributes.events {
            self.reactivity.push(quote! {
                let id = Rc::clone(&id);
                let closure = wasm_bindgen::prelude::Closure::<dyn FnMut()>::new(#callback);
                quux::dom::get_reactive_element(*id, #id)
                    .add_event_listener_with_callback(#event, closure.as_ref().unchecked_ref())
                    .expect_internal("add event");
                closure.forget();
            });
        }
        for expression in attributes.reactive_classes {
            self.reactivity.push(quote! {
                let (store, mapping, class_name) = #expression;
                let store = quux::store::Store::clone(store);
                let id = Rc::clone(&id);
                let class_list = quux::dom::get_reactive_element(*id, #id).class_list();
                store.on_change(move |previous, current| if mapping(std::clone::Clone::clone(current)) {
                    class_list.add_1(class_name).unwrap();
                } else {
                    class_list.remove_1(class_name).unwrap();
                })
            });
        }
    }

    fn add_child_data(&mut self, children: Items) {
        for child in children.items {
            let Self {
                mut components,
                mut reactivity,
                mut for_loops,
            } = child.into();
            self.components.append(&mut components);
            self.reactivity.append(&mut reactivity);
            self.for_loops.append(&mut for_loops);
        }
    }

    fn add_store_data(&mut self, ReactiveStore(store): &ReactiveStore, id: u64) {
        self.reactivity.push(quote! {
            let id = Rc::clone(&id);
            #store.on_change(move |_, new| {
                let element = quux::dom::get_reactive_element(*id, #id);
                quux::dom::as_html_element(element)
                    .set_inner_text(&std::string::ToString::to_string(new));
            });
        });
    }
}

pub fn generate(tree: &View) -> TokenStream {
    let View { context, element } = tree.clone();

    let Data {
        components,
        reactivity,
        for_loops,
        ..
    } = element.into();

    let for_loop_code: TokenStream = for_loops
        .into_iter()
        .enumerate()
        .map(|(index, mut for_loop)| for_loop.reactivity(index))
        .collect();

    let components = components
        .into_iter()
        .enumerate()
        // TODO: remove need to keep index in sync with server
        .map(|(index, component)| {
            let index = syn::Index::from(index);
            let binding = component.binding.map_or_else(TokenStream::new, |binding| {
                quote! {
                    #binding = component
                }
            });

            quote! {
                {
                    let child = children.#index;
                    let component = child.render().component;
                    #binding;
                }
            }
        });
    let tokens = quote! {
        use wasm_bindgen::JsCast;
        use quux::errors::MapInternal;
        use std::rc::Rc;
        use quux::component::Component;
        let children = #context.components;
        let mut for_loop_components = #context.for_loop_components;
        let id = Rc::new(#context.id);
        #(#components);*;
        #for_loop_code;
        #({ #reactivity });*;
        quux::view::output::Client::new(self)
    };
    tokens
}
