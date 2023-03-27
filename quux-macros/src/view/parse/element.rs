use super::internal::prelude::*;
use crate::view::generate::Html;

use std::sync::atomic::AtomicU64;
static ID: AtomicU64 = AtomicU64::new(0);

pub mod attributes;
pub mod children;

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
        let attributes: Attributes = input.parse()?;
        let id = attributes.id;
        Ok(Self {
            tag_name,
            attributes,
            children: Children::parse(input, id)?,
            ..Default::default()
        })
    }
}

pub mod prelude {
    pub use super::{
        attributes::Attributes,
        children::{Children, ForLoop, ForLoopIterable, Items, ReactiveStore},
        Element,
    };
}
