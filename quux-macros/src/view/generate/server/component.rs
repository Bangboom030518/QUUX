// TODO: Can we remove scope id now ??!1!?

use super::Html;
use crate::view::parse::prelude::*;
use quote::quote;

impl From<Component> for Html {
    fn from(value: Component) -> Self {
        let name = &value.name;
        let props = &value.props;
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
