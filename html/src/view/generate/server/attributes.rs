use std::collections::HashMap;

use quote::quote;
use syn::Expr;

use crate::view::parse::{Attribute, AttributeValue};

#[derive(Default)]
pub struct Attributes {
    pub keys: Vec<String>,
    pub values: Vec<Expr>,
    pub reactive: bool,
    pub reactive_attributes: HashMap<String, Expr>,
}

impl Attributes {
    pub fn add_scoped_id(&mut self, id: &str) {
        if self.reactive {
            self.keys.push(String::from("data-quux-scoped-id"));
            self.values.push(
                syn::parse(quote! { #id }.into())
                    .expect("Couldn't parse `id` tokens as expression (quux internal error)"),
            );
        }
    }

    pub fn add_entry(&mut self, key: String, value: Expr) {
        self.keys.push(key);
        self.values.push(value);
    }

    pub fn add_static_value(&mut self, key: String, value: Expr) {
        if key.starts_with("on:") {
            self.reactive = true;
        } else {
            self.add_entry(key, value);
        }
    }

    pub fn add_reactive_value(&mut self, key: String, value: &Expr) {
        
        self.reactive_attributes.insert(key.clone(), value.clone());
        self.add_entry(
            key,
            syn::parse::<Expr>(
                quote! {
                    quux::Store::get(#value)
                }
                .into(),
            )
            .expect("failed to parse `quux::Store::get(#value)` (QUUX internal)"),
        );
    }

    fn add_attribute(&mut self, Attribute { key, value }: Attribute) {
        match value {
            AttributeValue::Static(value) => self.add_static_value(key, value),
            AttributeValue::Reactive(value) => self.add_reactive_value(key, &value),
        }
    }
}

impl From<Vec<Attribute>> for Attributes {
    fn from(attributes: Vec<Attribute>) -> Self {
        let mut result = Self::default();
        for attribute in attributes {
            result.add_attribute(attribute);
        }
        result
    }
}
