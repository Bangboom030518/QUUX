use super::GLOBAL_ID;
use crate::view::parse::{Attribute, AttributeValue, Children, Component, Element, Item, Prop};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use shared::generate_id;
use std::collections::HashMap;
use std::sync::atomic::Ordering::Relaxed;
use syn::Expr;

struct Attributes {
    keys: Vec<String>,
    values: Vec<Expr>,
    dyn_attributes: HashMap<String, Expr>,
}

impl From<Vec<Attribute>> for Attributes {
    fn from(attributes: Vec<Attribute>) -> Self {
        let mut dyn_attributes = HashMap::new();
        let (keys, values): (Vec<_>, Vec<_>) = attributes
            .into_iter()
            .map(|Attribute { key, value }| match value {
                AttributeValue::Static(expr) => (key, expr),
                AttributeValue::Reactive(expr) => {
                    dyn_attributes.insert(key.clone(), expr.clone());
                    (
                        key,
                        syn::parse::<Expr>(
                            quote! {
                                quux::Store::get(#expr)
                            }
                            .into(),
                        )
                        .expect("failed to parse `quux::Store::get(#ident)` (QUUX internal)"),
                    )
                }
            })
            .unzip();
        Self {
            keys,
            values,
            dyn_attributes,
        }
    }
}

struct Data {
    /// tokens generating static SSR'd html
    html: TokenStream,
    /// tokens generating a `RenderContext` struct
    component_nodes: Vec<TokenStream>,
    /// the component which must be inserted into the view
    component_constructors: Vec<TokenStream>,
}

impl From<Item> for Data {
    /// Generates data for a single item in a view
    fn from(item: Item) -> Self {
        match item {
            Item::Element(element) => element.into(),
            Item::Component(component) => component.into(),
            Item::Expression(expression) => expression.into(),
        }
    }
}

impl From<Element> for Data {
    fn from(
        Element {
            tag_name,
            attributes,
            children,
        }: Element,
    ) -> Self {
        // TODO: deal with reactive stores as attribute values
        let mut attributes = attributes;

        let (html, component_nodes, component_constructors) =
            match children {
                Children::Children(children) => Self::from_element_children(children),
                Children::ReactiveStore(store) => {
                    let id = GLOBAL_ID.fetch_add(1, Relaxed);
                    attributes.push(Attribute {
                        key: String::from("data-quux-scoped-id"),
                        value: AttributeValue::Static(syn::parse(quote! { #id }.into()).expect(
                            "Couldn't parse `id` tokens as expression (quux internal error)",
                        )),
                    });
                    (Self::from_reactive_store(store), Vec::new(), Vec::new())
                }
            };

        let Attributes {
            keys,
            values: attribute_values,
            dyn_attributes,
        } = attributes.into();

        let html_string = &format!(
            "<{0} {1}>{{}}</{0}>",
            tag_name,
            keys.into_iter()
                .map(|key| format!("{key}=\"{{}}\""))
                .collect::<String>(),
        );

        let html = if attribute_values.is_empty() {
            quote! {
                format!(#html_string, #html)
            }
        } else {
            quote! {
                format!(#html_string, #(#attribute_values),*, #html)
            }
        };

        Self {
            html,
            component_constructors,
            component_nodes,
        }
    }
}

impl Data {
    fn from_element_children(
        children: Vec<Item>,
    ) -> (TokenStream, Vec<TokenStream>, Vec<TokenStream>) {
        let (mut html, (component_nodes, component_constructors)): (Vec<_>, (Vec<_>, Vec<_>)) =
            children
                .into_iter()
                .map(|item| {
                    let Self {
                        component_nodes,
                        html,
                        component_constructors,
                    } = Self::from(item);
                    (quote! { &#html }, (component_nodes, component_constructors))
                })
                .unzip();

        html.insert(0, quote! { String::new() });
        (
            quote!(#(#html)+*),
            component_nodes.concat(),
            component_constructors.concat(),
        )
    }

    fn from_reactive_store(store: Expr) -> TokenStream {
        quote!(shared::Store::get(&#store))
    }
}

impl From<Component> for Data {
    fn from(Component { name, props }: Component) -> Self {
        let component_id = generate_id();
        let component_ident = format_ident!("component_{}", component_id);
        let rendered_component_ident = format_ident!("rendered_component_{}", component_id);
        let props = props.into_iter().map(|Prop { key, value }| {
            quote! { #key : #value }
        });

        Self {
            html: quote! { #rendered_component_ident.html },
            component_nodes: vec![quote! {
                shared::ClientComponentNode {
                    component: shared::postcard::to_stdvec(&#component_ident).expect("Couldn't serialize component tree (QUUX internal)"),
                    render_context: shared::RenderContext {
                        id: shared::generate_id(),
                        children: Vec::new(),
                    },
                }
            }],
            component_constructors: vec![quote! {
                let #component_ident = #name ::init(<#name as shared::Component>::Props {
                    #(#props),*
                });
                let #rendered_component_ident = #component_ident.render();
            }],
        }
    }
}

impl From<Expr> for Data {
    fn from(expression: Expr) -> Self {
        Self {
            html: quote! {
                #expression.to_string()
            },
            component_nodes: Vec::new(),
            component_constructors: Vec::new(),
        }
    }
}

pub fn generate(tree: &Element) -> TokenStream {
    let mut tree = tree.clone();
    tree.attributes.push(Attribute {
        key: "data-quux-scope-id".to_string(),
        value: AttributeValue::Static(
            syn::parse(quote! { scope_id }.into())
                .expect("Couldn't parse `scope_id` as Expr (quux internal error)"),
        ),
    });
    let Data {
        html,
        component_nodes,
        component_constructors,
    } = Item::Element(tree).into();

    let tokens = quote! {
        let scope_id = shared::generate_id();
        #(#component_constructors)*
        shared::RenderData {
            html: #html,
            component_node: shared::ClientComponentNode {
                component: shared::postcard::to_stdvec(self).expect("Couldn't serialize component (quux internal error)"),
                render_context: shared::RenderContext {
                    id: scope_id,
                    children: vec![
                        #(#component_nodes),*
                    ],
                }
            }
        }
    };
    std::fs::write(
        "expansion-server.rs",
        quote! {fn main() {#tokens}}.to_string(),
    )
    .unwrap();
    tokens
}
