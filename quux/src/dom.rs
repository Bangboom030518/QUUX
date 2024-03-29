use super::errors::MapInternal;
use wasm_bindgen::prelude::*;

// TODO: consider moving these functions to more relevant locations

/// # Panics
/// If the element does not exist, it will panic.
/// It is up to the caller to ensure this is not the case.
#[must_use]
pub fn get_reactive_element(id: u64) -> web_sys::Element {
    query_selector(&format!("[data-quux-id='{id}']"))
}

/// # Panics
/// If the element does not exist, it will panic. It is up to the caller to ensure this is not the case.
#[must_use]
pub fn get_reactive_for_loop_element(
    parent_id: u64,
    for_loop_id: u64,
    index: usize,
) -> web_sys::Element {
    query_selector(&format!(
        "[data-quux-for-id='{parent_id}.{for_loop_id}.{index}']"
    ))
}

#[must_use]
pub fn query_selector(selector: &str) -> web_sys::Element {
    let error_message = format!("get element with selector ({selector})");
    document()
        .query_selector(selector)
        .expect_internal(&error_message)
        .expect_internal(&error_message)
}

#[must_use]
pub fn document() -> web_sys::Document {
    web_sys::window()
        .expect_internal("get window")
        .document()
        .expect_internal("get document")
}

#[must_use]
pub fn as_html_element(element: web_sys::Element) -> web_sys::HtmlElement {
    wasm_bindgen::JsCast::dyn_into(element).expect_internal("cast `Element` to `HTMLElement`")
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(input: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (quux::dom::log(&format_args!($($t)*).to_string()))
}

pub use console_log;
