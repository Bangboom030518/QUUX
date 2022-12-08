fn main()
{
    use std :: rc :: Rc ; use wasm_bindgen :: JsCast ; let mut children =
    context.children.into_iter() ; let scope_id = Rc :: new(context.id) ;
    {
        let child =
        children.next().expect("Client and server child lists don't match") ;
        let mut component : QUUXInitialise = shared :: postcard ::
        from_bytes(& child.component).expect("Couldn't deserialize component")
        ; component.render(child.render_context) ;
    }
    {
        let scope_id = Rc :: clone(& scope_id) ; let closure = wasm_bindgen ::
        prelude :: Closure :: < dyn FnMut() > ::
        new(move | |
        { let count = count.borrow_mut() ; count.set(count.get() + 1) ; }) ;
        web_sys ::
        window().expect("Failed to get window (quux internal error)").document().expect("Failed to get document (quux internal error)").query_selector(&
        format!
        ("[data-quux-scope-id='{}'] [data-quux-scoped-id='{}']", scope_id,
        "3")).expect("Failed to get element with scoped id (quux internal error)").expect("Failed to get element with scoped id (quux internal error)").add_event_listener_with_callback("click",
        closure.as_ref().unchecked_ref()).expect("Failed to add event (quux internal error)")
        ; closure.forget() ;
    } ;
    {
        let scope_id = Rc :: clone(& scope_id) ; shared :: Store ::
        on_change(& mut self.count, move | _, new |
        {
            wasm_bindgen :: JsCast :: dyn_into :: < web_sys :: HtmlElement >
            (web_sys ::
            window().expect("Failed to get window (quux internal error)").document().expect("Failed to get document (quux internal error)").query_selector(&
            format!
            ("[data-quux-scope-id='{}'] [data-quux-scoped-id='{}']", scope_id,
            "3")).expect("Failed to get element with scoped id (quux internal error)").expect("Failed to get element with scoped id (quux internal error)")).expect("`JSCast` from `Element` to `HTMLElement` (quux internal error)").set_inner_text(&
            std :: string :: ToString :: to_string(new))
        }) ;
    }
}