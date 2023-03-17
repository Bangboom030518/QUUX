use super::super::internal::prelude::*;
use crate::view::parse::prelude::*;

impl From<ReactiveStore> for syn::Expr {
    fn from(ReactiveStore(store): ReactiveStore) -> Self {
        parse_quote! { #store.get() }
    }
}

impl From<Item> for syn::Expr {
    fn from(value: Item) -> Self {
        let Html { html, .. } = value.into();
        parse_quote! { &#html }
    }
}

impl Items {
    /// Generates the body of an element.
    pub fn html_body(&self) -> syn::Expr {
        if self.items.is_empty() {
            return parse_quote! {
                String::new()
            };
        }
        let html = &self.items;
        parse_quote! {
            String::new() + #(#html)+*
        }
    }
}

const SELF_CLOSING_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "source", "source",
    "track", "wbr",
];

fn is_self_closing(tag_name: &str) -> bool {
    SELF_CLOSING_ELEMENTS.contains(&tag_name.to_lowercase().as_str())
}

impl Element {
    /// Generates the html body for an element.
    /// Sets `self.attributes.element_needs_id` if necessary
    fn html_body(&mut self) -> syn::Expr {
        if !matches!(&self.children, Children::Items(items) if items.items.is_empty()) {
            assert!(
                !is_self_closing(&self.tag_name),
                "Self-closing elements cannot contain children"
            );
        }
        match self.children.clone() {
            Children::Items(items) => items.html_body(),
            Children::ReactiveStore(store) => {
                self.attributes.element_needs_id = true;
                store.into()
            }
            Children::ForLoop(for_loop) => for_loop.tokens(self.attributes.id),
        }
    }

    pub fn insert_attribute(&mut self, key: &str, value: Expr) -> Option<Expr> {
        self.attributes.insert_static(key, value)
    }
}

impl From<Element> for Html {
    fn from(mut value: Element) -> Self {
        let attributes = value.attributes.clone();
        let tag_name = value.tag_name.clone();

        let html = if is_self_closing(&tag_name) {
            parse_quote! {
                format!("<{} {} />", #tag_name, #attributes)
            }
        } else {
            let body = value.html_body();
            parse_quote! {
                format!("<{0} {1}>{2}</{0}>", #tag_name, #attributes, #body)
            }
        };
        Self {
            html,
            components: todo!(),
            for_loop_components: todo!(),
        }
    }
}
