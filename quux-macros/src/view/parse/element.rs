use super::internal::prelude::*;
use crate::parse;
use attribute::Attribute;
pub use children::{Children, ForLoop};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;

pub mod attribute;
pub mod children;

#[derive(Default, Clone)]
pub struct Attributes {
    pub attributes: HashMap<String, Expr>,
    pub element_needs_id: bool,
    pub reactive_attributes: HashMap<String, Expr>,
}

impl Attributes {
    /// Adds a static attribute.
    /// If it's an event listener, the attribute will be ignored and  reactive will be set to true
    pub fn insert_static(&mut self, key: String, value: Expr) -> Option<Expr> {
        if key.starts_with("on:") || key == "class:active-when" {
            self.element_needs_id = true;
            None
        } else {
            self.attributes.insert(key, value)
        }
    }

    /// Adds a reactive attribute, setting it to the initial value of the store.
    pub fn insert_reactive(&mut self, key: String, value: &Expr) -> Option<Expr> {
        self.reactive_attributes.insert(key.clone(), value.clone());
        self.attributes.insert(
            key,
            parse(quote! {
                #value.get()
            }),
        )
    }

    /// Adds an `Attribute` to the list
    fn add_attribute(&mut self, Attribute { key, value }: Attribute) {
        match value {
            attribute::Value::Static(value) => {
                assert!(
                    self.insert_static(key, value).is_none(),
                    "Duplicate attributes found!"
                );
            }
            attribute::Value::Reactive(value) => {
                assert!(
                    self.insert_reactive(key, &value).is_none(),
                    "Duplicate attributes found!"
                );
            }
        };
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

#[derive(Clone, Default)]
pub struct Element {
    pub tag_name: String,
    pub attributes: Attributes,
    pub children: Children,
    pub component_initialisation_code: GenerationData,
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag_name = parse_html_ident(input)?;

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

// TODO: rename?
/// The generation code for an item
#[derive(Clone, Default)]
pub struct GenerationData {
    pub html: TokenStream,
}
