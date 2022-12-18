use super::errors::MapInternal;

#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn get_reactive_element(scope_id: &str, scoped_id: &str) -> web_sys::Element {
    get_document()
        .query_selector(&format!(
            "[data-quux-scope-id='{scope_id}'] [data-quux-scoped-id='{scoped_id}']"
        ))
        .expect_internal("get element with scoped id")
        .expect_internal("get element with scoped id")
}

#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn get_document() -> web_sys::Document {
    web_sys::window()
        .expect_internal("get window")
        .document()
        .expect_internal("get document")
}

#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn as_html_element(element: web_sys::Element) -> web_sys::HtmlElement {
    wasm_bindgen::JsCast::dyn_into(element).expect_internal("cast `Element` to `HTMLElement`")
}
