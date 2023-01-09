use std::collections::HashMap;

use super::parse;
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
    /// Add's the scoped id attribute with the value of `id`, if `attributes.reactive` is true. If the element is not reactive, nothing is added.
    pub fn add_scoped_id(&mut self, id: &str) {
        if self.reactive {
            self.keys.push(String::from("data-quux-scoped-id"));
            self.values.push(parse(quote! { #id }));
        }
    }

    /// Add's an attribute with a key of `key` and a value of `value`
    pub fn add_entry(&mut self, key: String, value: Expr) {
        self.keys.push(key);
        self.values.push(value);
    }

    /// Add's a static attribute.
    /// If it's an event listener, the attribute will be ignored and  reactive will be set to true
    pub fn add_static_value(&mut self, key: String, value: Expr) {
        if key.starts_with("on:") {
            self.reactive = true;
        } else {
            self.add_entry(key, value);
        }
    }

    /// Add's a reactive attribute, setting it to the initial value of the store.
    pub fn add_reactive_value(&mut self, key: String, value: &Expr) {
        self.reactive_attributes.insert(key.clone(), value.clone());
        self.add_entry(
            key,
            parse(quote! {
                #value.get()
            }),
        );
    }

    /// Add's an `Attribute`
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
