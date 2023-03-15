use super::super::internal::prelude::*;

impl Component {
    pub fn insert_for_loop_id(&mut self, id: u64) -> Option<u64> {
        if self.for_loop_id.is_some() {
            self.for_loop_id
        } else {
            self.for_loop_id = Some(id);
            None
        }
    }

    fn for_loop_id(&self) -> TokenStream {
        self.for_loop_id.map_or_else(
            || {
                quote! {
                    None
                }
            },
            |id| {
                let id = super::for_loop_id(id);
                quote! {
                    Some(#id)
                }
            },
        )
    }
}

impl From<Component> for Html {
    fn from(value: Component) -> Self {
        let name = &value.name;
        let props = &value.props;
        let for_loop_id = &value.for_loop_id();
        let html = quote! {
            {
                let component = <#name as quux::component::Component>::init(#props);
                component_id += 1;
                let id = component_id;
                let render_context = quux::render::Context {
                    id: id.clone(),
                    for_loop_id: #for_loop_id,
                    ..Default::default()
                };
                let rendered_component = quux::component::Component::render(&component, std::clone::Clone::clone(&render_context));
                // Push the component to the list of component for this view
                components.push(quux::render::ClientComponentNode {
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
