// TODO: Can we remove scope id now ??!1!?

use crate::view::parse::prelude::{element::GenerationData, *};
use component::Prop;
use proc_macro2::TokenStream;
use quote::quote;

// static ID: AtomicU64 = AtomicU64::new(0);

impl From<Prop> for TokenStream {
    fn from(Prop { key, value }: Prop) -> Self {
        quote! { #key: #value }
    }
}

impl From<Component> for GenerationData {
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
        Self { html }
    }
}

// pub struct Data {
//     name: Path,
//     props: Vec<Prop>,
//     component_ident: Ident,
//     rendered_component_ident: Ident,
//     component_context_ident: Ident,
// }

impl Component {
    // pub fn new(Component { name, props, .. }: Component) -> Self {
    //     let id = ID.fetch_add(1, Relaxed);
    //     let component_ident = format_ident!("component_{id}");
    //     let rendered_component_ident = format_ident!("rendered_component_{id}");
    //     let component_context_ident = format_ident!("component_context_{id}");
    //     Self {
    //         name,
    //         props,
    //         component_ident,
    //         rendered_component_ident,
    //         component_context_ident,
    //     }
    // }
    /// Generates the code to create and wrap the component in a `ClientComponentNode`
    // fn generate_node(&self) -> TokenStream {
    //     // let component = &self.component_ident;
    //     // let render_context = &self.component_context_ident;
    //     // let rendered_component = &self.rendered_component_ident;
    //     /*
    //         let #component_ident = <#name as quux::Component>::init(#props);
    //         let #component_context_ident = quux::RenderContext {
    //             id: quux::generate_id(),
    //             children: Vec::new(),
    //             for_loop_children: Vec::new()
    //         };
    //         let #rendered_component_ident = #component_ident.render(std::clone::Clone::clone(&#component_context_ident));
    //     */
    //     let name = &self.name;
    //     let props = self.generate_props();
    //     quote! {
    //         {
    //             let component = <#name as quux::Component>::init(#props);
    //             let render_context = quux::RenderContext {
    //                 id: quux::generate_id(),
    //                 children: Vec::new(),
    //                 for_loop_children: Vec::new()
    //             };
    //             let rendered_component = component.render(std::clone::Clone::clone(&render_context));
    //             quux::ClientComponentNode {
    //                 component: Self::ComponentEnum::from(component.clone()),
    //                 render_context: {
    //                     let render_context = rendered_component
    //                         .component_node
    //                         .render_context
    //                         .clone();
    //                     quux::RenderContext {
    //                         id: render_context.clone(),
    //                         ..render_context
    //                     }
    //                 },
    //             }
    //         }
    //     }
    // }

    // TODO: take a props, do not construct it
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
                <#name as quux::Component>::Props {
                    #(#props),*
                }
            }
        }
    }

    // fn generate_constructor(&self) -> TokenStream {
    //     let Self {
    //         component_ident,
    //         rendered_component_ident,
    //         component_context_ident,
    //         name,
    //         ..
    //     } = &self;
    //     let props = self.generate_props();
    //     quote! {
    //         let #component_ident = <#name as quux::Component>::init(#props);
    //         let #component_context_ident = quux::RenderContext {
    //             id: quux::generate_id(),
    //             children: Vec::new(),
    //             for_loop_children: Vec::new()
    //         };
    //         let #rendered_component_ident = #component_ident.render(std::clone::Clone::clone(&#component_context_ident));
    //     }
    // }
}

// impl From<Data> for super::Data {
//     fn from(data: Data) -> Self {
//         Self {
//             html: data.generate_html(),
//             component_nodes: vec![data.generate_node()],
//             component_constructors: vec![data.generate_constructor()],
//         }
//     }
// }
