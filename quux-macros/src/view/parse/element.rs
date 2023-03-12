use super::internal::prelude::*;
use crate::view::generate::Html;
use attribute::Attribute;
pub use children::{Children, ForLoop};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use syn::parse_quote;

static ID: AtomicU64 = AtomicU64::new(0);

pub mod attribute;
pub mod children;

#[derive(Default, Clone)]
pub struct Attributes {
    pub attributes: HashMap<String, Expr>,
    pub is_root: bool,
    pub element_needs_id: bool,
    pub reactive_attributes: HashMap<String, Expr>,
    pub events: HashMap<String, Expr>,
    pub reactive_classes: Vec<Expr>,
    pub id: u64,
}

impl Attributes {
    /// Adds a static attribute.
    /// If it's an event listener, the attribute will be added to events and reactive will be set to true
    pub fn insert_static(&mut self, key: &str, value: Expr) -> Option<Expr> {
        if let Some(event) = key.strip_prefix("on:") {
            self.element_needs_id = true;
            return self.events.insert(event.to_string(), value);
        }

        // TODO: should class name be included in key???
        if key == "class:active-when" {
            self.element_needs_id = true;
            self.reactive_classes.push(value);
            return None;
        }

        self.attributes.insert(key.to_string(), value)
    }

    /// Adds a reactive attribute, setting it to the initial value of the store.
    pub fn insert_reactive(&mut self, key: &str, value: &Expr) -> Option<Expr> {
        self.reactive_attributes
            .insert(key.to_string(), value.clone());
        self.attributes.insert(
            key.to_string(),
            parse_quote! {
                #value.get()
            },
        )
    }

    /// Adds an `Attribute` to the list
    fn add_attribute(&mut self, Attribute { key, value }: Attribute) {
        match value {
            attribute::Value::Static(value) => {
                assert!(
                    self.insert_static(&key, value).is_none(),
                    "Duplicate attributes found!"
                );
            }
            attribute::Value::Reactive(value) => {
                assert!(
                    self.insert_reactive(&key, &value).is_none(),
                    "Duplicate attributes found!"
                );
            }
        };
    }
}

impl From<Vec<Attribute>> for Attributes {
    fn from(attributes: Vec<Attribute>) -> Self {
        let mut result = Self {
            id: ID.fetch_add(1, Relaxed),
            ..Default::default()
        };
        for attribute in attributes {
            result.add_attribute(attribute);
        }
        result
    }
}

#[derive(Clone, Default)]
pub struct Element {
    pub tag_name: HtmlIdent,
    pub attributes: Attributes,
    pub children: Children,
    pub html: Html,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag_name = input.parse()?;

        let mut attributes = Vec::new();
        if input.peek(Paren) {
            let attributes_buffer;
            parenthesized!(attributes_buffer in input);
            while !attributes_buffer.is_empty() {
                attributes.push(attributes_buffer.parse()?);
                if attributes_buffer.peek(Token![,]) {
                    attributes_buffer.parse::<Token![,]>()?;
                } else if !attributes_buffer.is_empty() {
                    return Err(
                        attributes_buffer.error("Attributes should be seperated by commas, duh!")
                    );
                }
            }
        }

        let children;
        braced!(children in input);
        Ok(Self {
            tag_name,
            attributes: attributes.into(),
            children: children.parse()?,
            ..Default::default()
        })
    }
}
