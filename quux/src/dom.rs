use super::errors::MapInternal;
use wasm_bindgen::prelude::*;

#[must_use]
pub fn document() -> web_sys::Document {
    web_sys::window()
        .expect_internal("get window")
        .document()
        .expect_internal("get document")
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
