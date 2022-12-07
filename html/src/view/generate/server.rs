use super::GLOBAL_ID;
use crate::view::parse::{Attribute, AttributeValue, Children, Component, Element, Item, Prop};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use shared::generate_id;
use std::collections::HashMap;
use std::sync::atomic::Ordering::Relaxed;
use syn::Expr;

#[derive(Default)]
struct Attributes {
    keys: Vec<String>,
    values: Vec<Expr>,
    reactive: bool,
    reactive_attributes: HashMap<String, Expr>,
}

impl Attributes {
    fn add_entry(&mut self, key: String, value: Expr) {
        self.keys.push(key);
        self.values.push(value);
    }

    fn static_value(&mut self, key: String, value: Expr) {
        if key.starts_with("on:") {
            self.reactive = true;
        } else {
            self.add_entry(key, value);
        }
    }

    fn reactive_value(&mut self, key: String, value: &Expr) {
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
}

impl From<Vec<Attribute>> for Attributes {
    fn from(attributes: Vec<Attribute>) -> Self {
        let mut result = Self::default();
        for Attribute { key, value } in attributes {
            match value {
                AttributeValue::Static(value) => result.static_value(key, value),
                AttributeValue::Reactive(value) => result.reactive_value(key, &value)
            }
        }
        result
    }
}

#[derive(Default)]
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
        let id = GLOBAL_ID.fetch_add(1, Relaxed).to_string();

        let mut data = Self::default();

        // TODO: deal with reactive stores as attribute values
        let mut attributes: Attributes = attributes.into();

        match children {
            Children::Children(children) => data.add_child_data(children),
            Children::ReactiveStore(store) => {
                attributes.reactive = true;
                data.add_store_data(&store);
            }
        };

        data.add_attribute_data(attributes, &tag_name.to_string(), &id);

        data
    }
}

impl Data {
    fn add_child_data(&mut self, children: Vec<Item>) {
        let mut html: Vec<_> = children
            .into_iter()
            .map(|child| {
                let Self {
                    mut component_nodes,
                    html,
                    mut component_constructors,
                } = child.into();
                self.component_nodes.append(&mut component_nodes);
                self.component_constructors
                    .append(&mut component_constructors);
                quote! { &#html }
            })
            .collect();
        html.insert(0, quote! { String::new() });
        self.html = quote!(#(#html)+*);
    }

    fn add_attribute_data(
        &mut self,
        Attributes {
            mut keys,
            mut values,
            reactive_attributes: dyn_attributes,
            reactive,
        }: Attributes,
        tag_name: &str,
        id: &str,
    ) {
        if reactive {
            keys.push(String::from("data-quux-scoped-id"));
            values.push(
                syn::parse(quote! { #id }.into())
                    .expect("Couldn't parse `id` tokens as expression (quux internal error)"),
            );
        }

        let html_string = &format!(
            "<{0} {1}>{{}}</{0}>",
            tag_name,
            keys.into_iter()
                .map(|key| format!("{key}=\"{{}}\""))
                .collect::<String>(),
        );
        let html = &self.html;

        self.html = if values.is_empty() {
            quote! {
                format!(#html_string, #html)
            }
        } else {
            quote! {
                format!(#html_string, #(#values),*, #html)
            }
        };
    }

    fn add_store_data(&mut self, store: &Expr) {
        self.html = quote!(shared::Store::get(&#store));
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
