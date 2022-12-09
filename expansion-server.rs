fn main()
{
    let scope_id = shared :: generate_id() ; let component_0 = QUUXInitialise
    :: init(< QUUXInitialise as shared :: Component > :: Props {}) ; let
    rendered_component_0 = component_0.render() ; shared :: RenderData
    {
        html : format!
        ("<html lang=\"{}\"data-quux-scope-id=\"{}\">{}</html>", "en",
        scope_id, String :: new() + & format!
        ("<head >{}</head>", String :: new()) + & format!
        ("<body >{}</body>", String :: new() + & format!
        ("<button data-quux-scoped-id=\"{}\">{}</button>", "3", shared ::
        Store :: get(& Rc :: clone(& count).borrow())) + &
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
                    { id : shared :: generate_id(), children : Vec :: new(), }
                }],
            }
        }
    }
}