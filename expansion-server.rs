fn main()
{
    let scope_id = shared :: generate_id() ; let component_0 = QUUXInitialise
    :: init(< QUUXInitialise as shared :: Component > :: Props {}) ; let
    rendered_component_0 = component_0.render() ; shared :: RenderData
    {
        html : format!
        ("<html lang=\"{}\"data-quux-scope-id=\"{}\">{}</html>", "en",
        scope_id, String :: new() + & format!
        ("<head >{}</head>", String :: new() + & format!
        ("<style >{}</style>", String :: new() + &
        "
                            button {
                                background: red;
                                width: 100%;
                            }
                        ".to_string()))
        + & format!
        ("<body >{}</body>", String :: new() + & format!
        ("<button style=\"{}\"data-quux-scoped-id=\"{}\">{}</button>",
        "background: red;", 0u64, shared :: Store :: get(& self.count)) + &
        rendered_component_0.html)), component_node : shared ::
        ClientComponentNode
        {
            component : shared :: postcard ::
            to_stdvec(self).expect("Couldn't serialize component (quux internal error)"),
            render_context : shared :: RenderContext
            {
                id : scope_id, children : vec!
                [shared :: ClientComponentNode
                {
                    component : shared :: postcard ::
                    to_stdvec(&
                    component_0).expect("Couldn't serialize component tree (QUUX internal)"),
                    render_context : shared :: RenderContext
                    { id : shared :: generate_id(), children : Vec :: new(), },
                }],
            }
        }
    }
}