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
        let Component {
            name, props, ident, ..
        } = &value;
        let for_loop_id = &value.for_loop_id();
        let html = parse_quote! {
            {
                let component = <#name as quux::component::Init>::init(#props);
                component_id += 1;
                let id = component_id;
                let render_context = ServerContext::new(id.clone(), #for_loop_id);
                // TODO: remove clone
                let Output { component_node: SerializedComponent { component, render_context }, html } = component.render(render_context.clone());
                #ident = SerializedComponent::new(component, render_context);
                html
            }
        };
        Self {
            html,
            components: Components(vec![value]),
            for_loop_components: ForLoops::default(),
        }
    }
}
