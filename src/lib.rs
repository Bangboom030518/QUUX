#![feature(more_qualified_paths)]
use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, QUUXInitialise, RenderData, Store};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn init_app() {
    log("Hello!")
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
