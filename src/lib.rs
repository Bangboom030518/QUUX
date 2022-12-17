#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]
use html::view;
use serde::{Deserialize, Serialize};
use shared::{Component, QUUXInitialise, Store};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
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
    let root_component: App =
        shared::postcard::from_bytes(&tree.component).expect("failed to deserialize Component");
    root_component.render(tree.render_context);
}

#[derive(Serialize, Deserialize)]
pub struct App {
    count: Store<'static, u32>,
}

impl<'a> Component<'a> for App {
    type Props = ();

    fn init(_props: Self::Props) -> Self {
        Self {
            count: Store::new(0),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render(&self) -> shared::RenderData {
        view! {
            html(lang="en") {
                head {
                }
                body {
                    button(on:click=|| log("HELLO!!!!!!")) {
                        $self.count
                    }
                    @QUUXInitialise
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn render(self, context: shared::RenderContext) {
        use std::cell::RefCell;
        use std::rc::Rc;
        let count = Rc::new(RefCell::new(self.count));
        let button_count = Rc::clone(&count);
        let interpolated_count = Rc::clone(&count);
        view! {
            html(lang="en") {
                head {
                }
                body {
                    button(on:click=move || {
                        let before = {
                            let before_store = button_count.borrow();
                            *before_store.get()
                        };
                        let mut count = button_count.borrow_mut();
                        count.set(before + 1);
                    }) {
                        $interpolated_count.borrow_mut()
                    }
                    @QUUXInitialise
                }
            }
        };
    }
}
