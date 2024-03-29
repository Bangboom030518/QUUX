use super::super::internal::prelude::*;
use crate::view::parse::prelude::*;

impl ToTokens for ReactiveStore {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self(store) = self;
        quote! { #store.get() }.to_tokens(tokens);
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Html { html, .. } = self.clone().into();
        html.to_tokens(tokens);
    }
}

impl From<Children> for Html {
    fn from(value: Children) -> Self {
        match value {
            Children::ForLoop(for_loop) => Self::from(for_loop),
            Children::If(if_expr) => Self::from(if_expr),
            Children::Items(items) => Self::from(items),
            Children::Match(match_expr) => Self::from(match_expr),
            Children::ReactiveStore(store) => Self::from(store),
        }
    }
}

impl ToTokens for Children {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::ForLoop(for_loop) => for_loop.to_tokens(tokens),
            Self::If(if_expr) => if_expr.to_tokens(tokens),
            Self::Items(items) => items.to_tokens(tokens),
            Self::Match(match_expr) => match_expr.to_tokens(tokens),
            Self::ReactiveStore(store) => store.to_tokens(tokens),
        }
    }
}

impl From<Items> for Html {
    fn from(value: Items) -> Self {
        if value.items.is_empty() {
            return Self::default();
        }

        let (html, types): (Vec<_>, Vec<_>) = value
            .items
            .iter()
            .cloned()
            .map(|item| {
                let Self { html, ty } = item.into();
                (html, ty)
            })
            .unzip();

        Self {
            html: parse_quote! {
                String::new() + #(&#html)+*
            },
            ty: parse_quote! {
                (#(#types,)*)
            },
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
    pub fn insert_attribute(&mut self, key: &str, value: Expr) -> Option<Expr> {
        self.attributes.insert_static(key, value)
    }
}

impl ToTokens for Element {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            tag_name,
            attributes,
            children,
            ..
        } = self;
        quote! {
            quux::tree::Element::new(#tag_name, #attributes, #children)
        }
        .to_tokens(tokens);
    }
}

impl From<Element> for Html {
    fn from(mut value: Element) -> Self {
        let attributes = value.attributes.clone();
        let tag_name = value.tag_name.clone();
        if is_self_closing(&tag_name) {
            assert!(
                matches!(&value.children, Children::Items(items) if items.items.is_empty()),
                "Self-closing elements cannot contain children"
            );
            Self {
                html: parse_quote! {
                    format!("<{} {} />", #tag_name, #attributes)
                },
                ..Default::default()
            }
        } else {
            let html: Self = match value.children {
                Children::Items(items) => items.into(),
                Children::ReactiveStore(store) => {
                    value.attributes.element_needs_id = true;
                    Self {
                        html: parse_quote! { #store },
                        ..Default::default()
                    }
                }
                Children::ForLoop(for_loop) => for_loop.html(),
                Children::If(expr) => expr.html(),
                Children::Match(expr) => expr.html(),
            };
            let body = html.html;
            Self {
                html: parse_quote! {
                    format!("<{0} {1}>{2}</{0}>", #tag_name, #attributes, #body)
                },
                ..html
            }
        }
    }
}
