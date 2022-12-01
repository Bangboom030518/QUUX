fn main() {
    {
        let component_0 = QUUXInitialise::init(<QUUXInitialise as shared::Component>::Props {});
        let rendered_component_0 = component_0.render();
        shared::RenderData {
            html: format!(
                "<html lang=\"{}\">{}</html>",
                "en",
                String::new()
                    + &format!("<head >{}</head>", String::new())
                    + &format!(
                        "<body >{}</body>",
                        String::new()
                            + &format!(
                                "<button >{}</button>",
                                String::new() + &self.count.to_string()
                            )
                            + &rendered_component_0.html
                    )
            ),
            render_context: shared::RenderContext {
                id: shared::generate_id(),
                children: vec![shared::ClientComponentNode {
                    component: shared::postcard::to_stdvec(&component_0)
                        .expect("Couldn't serialize component tree (internal)"),
                    render_context: shared::RenderContext {
                        id: shared::generate_id(),
                        children: Vec::new(),
                    },
                    static_id: "0".to_string(),
                }],
            },
        }
    }
}
