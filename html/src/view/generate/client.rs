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
    fn from(item: Item) -> Self {
        match item {
            Item::Component(component) => Self {
                components: vec![component.name],
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
            scoped_id: GLOBAL_ID.fetch_add(1, Relaxed).to_string(),
            ..Default::default()
        };
        data.add_event_data(attributes);
        match children {
            Children::Children(children) => data.add_child_data(children),
            Children::ReactiveStore(store) => data.add_store_data(&store),
        };
        data
    }
}

impl Data {
    fn add_attribute_data(&mut self, Attribute { key, value }: Attribute) {
        let AttributeValue::Static(value) = value else {
            return
        };

        let Some(event_name) = key.strip_prefix("on:") else {
            return
        };

        let scoped_id = self.scoped_id.as_str();

        self.reactivity.push(quote! {
            let scope_id = Rc::clone(&scope_id);
            let closure = wasm_bindgen::prelude::Closure::<dyn FnMut()>::new(#value);
            shared::dom::get_reactive_element(&*scope_id, #scoped_id)
                .add_event_listener_with_callback(#event_name, closure.as_ref().unchecked_ref())
                .expect_internal("add event");
            closure.forget();
        });
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
        let scoped_id = self.scoped_id.as_str();
        self.reactivity.push(quote! {
            let scope_id = Rc::clone(&scope_id);
            shared::Store::on_change(&mut #store, move |_, new| {
                let element = shared::dom::get_reactive_element(&*scope_id, #scoped_id);
                shared::dom::as_html_element(element)
                    .set_inner_text(&std::string::ToString::to_string(new));
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
    let tokens = quote! {
        use std::rc::Rc;
        use wasm_bindgen::JsCast;
        use shared::errors::MapInternal;
        let mut children = context.children.into_iter();
        let scope_id = Rc::new(context.id);
        #({
            let child = children.next().expect_internal("retrieve all child data: client and server child lists don't match");
            let mut component: #components = shared::postcard::from_bytes(&child.component).expect("Couldn't deserialize component");
            component.render(child.render_context);
        })*
        #({ #reactivity });*
    };
    std::fs::write(
        "expansion-client.rs",
        quote! {fn main() {#tokens}}.to_string(),
    )
    .unwrap();
    tokens
}
