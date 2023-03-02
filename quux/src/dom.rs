use super::errors::MapInternal;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn get_reactive_element(parent_id: &str, child_id: &str) -> web_sys::Element {
    let error_message =
        format!("get element with selector ([data-quux-scoped-id='{parent_id}.{child_id}'])");
    get_document()
        .query_selector(&format!("[data-quux-scoped-id='{parent_id}.{child_id}']"))
        .expect_internal(&error_message)
        .expect_internal(&error_message)
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(input: &str);
}

// TODO: remove `unsafe`
#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (unsafe { quux::dom::log(&format_args!($($t)*).to_string()) })
}

pub use console_log;
