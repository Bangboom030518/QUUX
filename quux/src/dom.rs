use super::errors::MapInternal;
use wasm_bindgen::prelude::*;

/// # Panics
/// If the element does not exist, it will panic.
/// It is up to the caller to ensure this is not the case.
#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn get_reactive_element(parent_id: &str, child_id: &str) -> web_sys::Element {
    let error_message =
        format!("get element with selector ([data-quux-scoped-id='{parent_id}.{child_id}'])");
    document()
        .query_selector(&format!("[data-quux-scoped-id='{parent_id}.{child_id}']"))
        .expect_internal(&error_message)
        .expect_internal(&error_message)
}

/// # Panics
/// If the element does not exist, it will panic.
/// It is up to the caller to ensure this is not the case.
#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn get_reactive_for_loop_element(
    parent_id: &str,
    for_loop_id: &str,
    index: usize,
) -> web_sys::Element {
    let selector = format!("[data-quux-for-id='{parent_id}.{for_loop_id}.{index}']");
    let error_message = format!("get element with selector ({selector})");
    document()
        .query_selector(&selector)
        .expect_internal(&error_message)
        .expect_internal(&error_message)
}

#[cfg(target_arch = "wasm32")]
#[must_use]
pub fn document() -> web_sys::Document {
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
