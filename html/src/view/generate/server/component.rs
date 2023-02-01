// TODO: Can we remove scope id now ??!1!?

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use shared::generate_id;
use syn::Path;
use crate::view::parse::prelude::*;
use component::Prop;

impl From<Prop> for TokenStream {
    fn from(Prop { key, value }: Prop) -> Self {
        quote! { #key: #value }
    }
}

pub struct Data {
    name: Path,
    props: Vec<Prop>,
    component_ident: Ident,
    rendered_component_ident: Ident,
    component_context_ident: Ident,
}

impl Data {
    pub fn new(Component { name, props, .. }: Component) -> Self {
        let id = generate_id();
        let component_ident = format_ident!("component_{id}");
        let rendered_component_ident = format_ident!("rendered_component_{id}");
        let component_context_ident = format_ident!("component_context_{id}");
        Self {
            name,
            props,
            component_ident,
            rendered_component_ident,
            component_context_ident,
        }
    }

    fn generate_html(&self) -> TokenStream {
        let component = &self.rendered_component_ident;
        quote! { #component.html }
    }

    fn generate_node(&self) -> TokenStream {
        let component = &self.component_ident;
        let render_context = &self.component_context_ident;
        let rendered_component = &self.rendered_component_ident;
        quote! {
            shared::ClientComponentNode {
                component: T::from(#component),
                render_context: shared::RenderContext {
                    id: #render_context.id,
                    children: #rendered_component
                        .component_node
                        .render_context
                        .children,
                },
            }
        }
    }

    fn generate_context(&self) -> TokenStream {
        quote! {
            shared::RenderContext {
                id: shared::generate_id(),
                children: Vec::new()
            }
        }
    }

    fn generate_props(&self) -> TokenStream {
        // TODO: remove `.cloned()`
        let props = self.props.iter().cloned().map::<TokenStream, _>(Prop::into);
        let name = &self.name;
        if props.is_empty() {
            quote! {
                ()
            }
        } else {
            quote! {
                <#name as shared::Component>::Props {
                    #(#props),*
                }
            }
        }
    }

    fn generate_constructor(&self) -> TokenStream {
        let Self {
            component_ident,
            rendered_component_ident,
            component_context_ident,
            name,
            ..
        } = &self;
        let props = self.generate_props();
        let context = self.generate_context();
        quote! {
            let #component_ident = <#name as shared::Component>::init(#props);
            let #component_context_ident = #context;
            let #rendered_component_ident = #component_ident.render(std::clone::Clone::clone(&#component_context_ident));
        }
    }
}

impl From<Data> for super::Data {
    fn from(data: Data) -> Self {
        Self {
            html: data.generate_html(),
            component_nodes: vec![data.generate_node()],
            component_constructors: vec![data.generate_constructor()],
        }
    }
}

impl From<Component> for super::Data {
    fn from(component: Component) -> Self {
        Data::new(component).into()
    }
}
