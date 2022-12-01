#![feature(more_qualified_paths)]
use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, QUUXInitialise, RenderData, Store};
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use web_sys::{window};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init_app() {
    let init_script = window().unwrap().document().unwrap().get_element_by_id("__quux_init_script__").expect("`__quux_init_script__` not found");
    let tree = init_script.get_attribute("data-quux-tree").expect("`__quux_init_script__` doesn't have a tree attached");
    let tree: shared::RenderContext = postcard::from_bytes(&base64::decode(tree).expect("Failed to decode tree as base64")).expect("Render context tree malformatted");
    log(tree.id);
}

#[derive(Serialize, Deserialize)]
pub struct App<'a> {
    count: Store<'a, u32>,
}

impl<'a> Component<'a> for App<'a> {
    type Props = ();

    fn init(_props: Self::Props) -> Self {
        Self {
            count: Store::new(0),
        }
    }
    fn render(&self) -> RenderData {
        view! {
            html(lang="en") {
                head {}
                body {
                    button {
                        { self.count }
                    }
                    @QUUXInitialise
                }
            }
        }
    }
}
