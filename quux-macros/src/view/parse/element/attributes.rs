use super::super::internal::prelude::*;
use syn::parse_quote;
use std::{sync::atomic::Ordering::Relaxed, collections::HashMap};
use super::ID;

#[derive(Clone)]
struct Attribute {
    key: HtmlIdent,
    value: Value,
}

impl Parse for Attribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { key, value })
    }
}

#[derive(Clone)]
pub enum Value {
    Reactive(Expr),
    Static(Expr),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Token![$]) {
            input.parse::<Token![$]>()?;
            Ok(Self::Reactive(input.parse()?))
        } else {
            Ok(Self::Static(input.parse()?))
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reactive(expr) => write!(f, "${}", expr.to_token_stream()),
            Self::Static(expr) => write!(f, "{}", expr.to_token_stream()),
        }
    }
}

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
            Value::Static(value) => {
                assert!(
                    self.insert_static(&key, value).is_none(),
                    "Duplicate attributes found!"
                );
            }
            Value::Reactive(value) => {
                assert!(
                    self.insert_reactive(&key, &value).is_none(),
                    "Duplicate attributes found!"
                );
            }
        };
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
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
        Ok(attributes.into())
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
