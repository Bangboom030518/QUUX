use super::Head;
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index;

impl Component for Index {
    fn render(self, context: Context<Self>) -> Output<Self> {
        type Component = Index;
        // view! {
        //     context,
        //     html(lang="en", magic=true) {
        //         @Head("QUUXLET - like Quizlet but gud".to_string())
        //         body {
        //             h1 {{ "Welcome to QUUXLET" }}
        //             @InitialisationScript(include_str!("../../dist/init.js"))
        //         }
        //     }
        // }
        let __self = self.clone();
        #[cfg(target_arch = "wasm32")]
        let render_server = {
            use quux::view::ServerContext;
            let context = ServerContext::<Self>::new(context.id, context.for_loop_id.clone());
            let __self = __self.clone();
            move || {
                use quux::{
                    component::Component as _,
                    view::{output, ClientContext, SerializedComponent, ServerContext},
                };
                let context = context;
                let id = context.id;
                let mut component_id = context.id;
                let for_loop_id = context.for_loop_id;
                let component_7: quux::view::SerializedComponent<Head>;
                let component_8: quux::view::SerializedComponent<Flashcards>;
                let component_9: quux::view::SerializedComponent<InitialisationScript>;
                output::Server::new(
                    &format!(
                        "<{0} {1}>{2}</{0}>",
                        "html",
                        String::new()
                            + &format!("{}=\"{}\"", "lang", "en")
                            + &format!("{}=\"{}\"", "magic", true)
                            + &if let Some(id) = for_loop_id {
                                format!("data-quux-for-id=\"{}\"", id)
                            } else {
                                String::new()
                            },
                        String::new()
                            + &{
                                let component = <Head as quux::component::Init>::init(
                                    "Flashcards - QUUX".to_string(),
                                );
                                component_id += 1;
                                let id = component_id;
                                let render_context = ServerContext::new(id.clone(), None);
                                let Output {
                                    component_node:
                                        SerializedComponent {
                                            component,
                                            render_context,
                                        },
                                    html,
                                } = component.render(render_context.clone());
                                component_7 = SerializedComponent::new(component, render_context);
                                html
                            }
                            + &format!(
                                "<{0} {1}>{2}</{0}>",
                                "body",
                                String::new(),
                                String::new()
                                    + &format!(
                                        "<{0} {1}>{2}</{0}>",
                                        "h1",
                                        String::new(),
                                        String::new() + &"Welcome to Quuxlet".to_string()
                                    )
                                    + &{
                                        let component = <Flashcards as quux::component::Init>::init(
                                            self.0.terms.clone(),
                                        );
                                        component_id += 1;
                                        let id = component_id;
                                        let render_context = ServerContext::new(id.clone(), None);
                                        let Output {
                                            component_node:
                                                SerializedComponent {
                                                    component,
                                                    render_context,
                                                },
                                            html,
                                        } = component.render(render_context.clone());
                                        component_8 =
                                            SerializedComponent::new(component, render_context);
                                        html
                                    }
                                    + &{
                                        let component =
                                            <InitialisationScript as quux::component::Init>::init(
                                                include_str!("../../dist/init.js"),
                                            );
                                        component_id += 1;
                                        let id = component_id;
                                        let render_context = ServerContext::new(id.clone(), None);
                                        let Output {
                                            component_node:
                                                SerializedComponent {
                                                    component,
                                                    render_context,
                                                },
                                            html,
                                        } = component.render(render_context.clone());
                                        component_9 =
                                            SerializedComponent::new(component, render_context);
                                        html
                                    }
                            )
                    ),
                    SerializedComponent::new(
                        __self,
                        ClientContext::new(id, None, (component_7, component_8, component_9), ()),
                    ),
                )
            }
        };
        impl quux::view::ComponentChildren for Component {
            type Components = (
                quux::view::SerializedComponent<Head>,
                quux::view::SerializedComponent<Flashcards>,
                quux::view::SerializedComponent<InitialisationScript>,
            );
            type ForLoopComponents = ();
        };
        quux::cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")]
            {
                {
                    use wasm_bindgen :: JsCast ; use quux :: errors :: MapInternal
                    ; use std :: rc :: Rc ; use quux :: component :: Component ;
                    let children = context.components ; let mut
                    for_loop_components = context.for_loop_components ; let id =
                    Rc :: new(context.id) ;
                    {
                        let child = children.0 ; let component =
                        child.render().component ; ;
                    } ;
                    {
                        let child = children.1 ; let component =
                        child.render().component ; ;
                    } ;
                    {
                        let child = children.2 ; let component =
                        child.render().component ; ;
                    } ; ; ; quux :: view :: output :: Client :: new(self)
                }
            } else { { render_server() } }
        }
    }
}
