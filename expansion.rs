fn main() {
    use std::rc::Rc;
    let mut children = context.children.into_iter();
    let scope_id = Rc::new(context.id);
    {
        let child = children
            .next()
            .expect("Client and server child lists don't match");
        let mut component: QUUXInitialise =
            shared::postcard::from_bytes(&child.component).expect("Couldn't deserialize component");
        component.render(child.render_context);
    }
    let scope_id_0 = Rc::clone(&scope_id);
    shared::Store::on_change(&mut self.count, move |_, new| {
        wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(
            web_sys::window()
                .expect("Failed to get window (quux internal error)")
                .document()
                .expect("Failed to get document (quux internal error)")
                .query_selector(&format!(
                    "[data-quux-scope-id='{}'] [data-quux-scoped-id='{}']",
                    scope_id_0, 0u64
                ))
                .expect("Failed to get element with scoped id (quux internal error)")
                .expect("Failed to get element with scoped id (quux internal error)"),
        )
        .expect("`JSCast` from `Element` to `HTMLElement` (quux internal error)")
        .set_inner_text(&std::string::ToString::to_string(new))
    });
    let scope_id_1 = Rc::clone(&scope_id);
    shared::Store::on_change(&mut self.count, move |_, new| {
        wasm_bindgen::JsCast::dyn_into::<web_sys::HtmlElement>(
            web_sys::window()
                .expect("Failed to get window (quux internal error)")
                .document()
                .expect("Failed to get document (quux internal error)")
                .query_selector(&format!(
                    "[data-quux-scope-id='{}'] [data-quux-scoped-id='{}']",
                    scope_id_1, 1u64
                ))
                .expect("Failed to get element with scoped id (quux internal error)")
                .expect("Failed to get element with scoped id (quux internal error)"),
        )
        .expect("`JSCast` from `Element` to `HTMLElement` (quux internal error)")
        .set_inner_text(&std::string::ToString::to_string(new))
    });
}
