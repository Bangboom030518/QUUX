// TODO: Can we remove scope id now ??!1!?

use super::Html;
use crate::view::parse::prelude::*;
use component::Prop;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

impl ToTokens for Prop {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Prop { key, value } = &self;
        tokens.extend(quote! { #key: #value })
    }
}

impl From<Component> for Html {
    fn from(value: Component) -> Self {
        let name = &value.name;
        let props = value.generate_props();
        let html = quote! {
            {
                let component = <#name as quux::Component>::init(#props);
                let render_context = quux::RenderContext {
                    id: quux::generate_id(),
                    children: Vec::new(),
                    for_loop_children: Vec::new()
                };
                let rendered_component = component.render(std::clone::Clone::clone(&render_context));
                // Push the component to the list of component for this view
                components.push(quux::ClientComponentNode {
                    component: Self::ComponentEnum::from(component.clone()),
                    render_context: rendered_component
                        .component_node
                        .render_context
                        .clone()
                    ,
                });
                rendered_component.html
            }
        };
        Self(html)
    }
}

impl Component {
    // TODO: take a props, do not construct it
    fn generate_props(&self) -> TokenStream {
        // TODO: remove `.cloned()`
        // let props = self.props.iter().cloned().map::<TokenStream, _>(Prop::into);
        let name = &self.name;
        let props = &self.props;
        if self.props.is_empty() {
            quote! {
                ()
            }
        } else {
            quote! {
                <#name as quux::Component>::Props {
                    #(#props),*
                }
            }
        }
    }
}
