use super::Html;
use crate::view::parse::prelude::*;
use quote::quote;
use syn::Expr;

impl Component {
    pub fn insert_for_loop_id(&mut self, value: Expr) -> Option<Expr> {
        if self.for_loop_id.is_some() {
            self.for_loop_id.clone()
        } else {
            self.for_loop_id = Some(value);
            None
        }
    }
}

impl From<Component> for Html {
    fn from(value: Component) -> Self {
        let name = &value.name;
        let props = &value.props;
        let for_loop_id = &value.for_loop_id.map_or_else(
            || {
                quote! {
                    None
                }
            },
            |id| {
                quote! {
                    Some(#id)
                }
            },
        );
        let html = quote! {
            {
                let component = <#name as quux::Component>::init(#props);
                let id = quux::generate_id();
                let render_context = quux::RenderContext {
                    id: id.clone(),
                    for_loop_id: #for_loop_id,
                    ..Default::default()
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
