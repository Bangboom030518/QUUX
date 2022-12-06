#![feature(more_qualified_paths, stmt_expr_attributes)]
// #![warn(clippy::pedantic, clippy::nursery)]
use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, QUUXInitialise, RenderData, Store};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn init_app() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let init_script = document()
        .get_element_by_id("__quux_init_script__")
        .expect("`__quux_init_script__` not found");

    let tree = init_script
        .get_attribute("data-quux-tree")
        .expect("`__quux_init_script__` doesn't have a tree attached");
    let tree: shared::ClientComponentNode =
        postcard::from_bytes(&base64::decode(tree).expect("Failed to decode tree as base64"))
            .expect("Render context tree malformatted");
    let mut root_component: App =
        shared::postcard::from_bytes(&tree.component).expect("failed to deserialize Component");
    root_component.render(tree.render_context);
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

    #[cfg(not(target_arch = "wasm32"))]
    fn render(&self) -> RenderData {
        view! {
            html(lang="en") {
                head {
                    style {
                        {"
                            button {
                                background: red;
                                width: 100%;
                            }
                        "}
                    }
                }
                body {
                    button(style="background: red;") {
                        $self.count
                    }
                    @QUUXInitialise
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn render(&mut self, context: shared::RenderContext) {
        log("The `render` method of `App` has been called.");
        view! {
            html(lang="en") {
                head {
                    style {
                        {"
                            button {
                                background: red;
                                width: 100%;
                            }
                        "}
                    }
                }
                body {
                    button(style="background: red;") {
                        $self.count
                    }
                    @QUUXInitialise
                }
            }
        };
        self.count.set(300);
    }
}
