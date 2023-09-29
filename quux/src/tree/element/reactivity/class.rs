#[cfg_client]
use super::Reactivity;
use crate::internal::prelude::*;

#[derive(Clone, Debug)]
pub struct Class {
    class: String,
    store: Store<bool>,
}

impl Class {
    #[must_use]
    pub fn new(class: &str, store: Store<bool>) -> Self {
        Self {
            class: class.to_string(),
            store,
        }
    }
}

#[cfg_client]
impl Reactivity for Class {
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>) {
        let class = self.class.to_string();
        // TODO: simpler method?
        self.store.on_change(move |_, &enabled| {
            let class_list = element.class_list();
            if enabled {
                class_list.add_1(&class).unwrap();
            } else {
                class_list.remove_1(&class).unwrap();
            }
        });
    }
}

#[macro_export]
macro_rules! event {
    ($closure:expr) => {{
        #[cfg(target_arch = "wasm32")]
        {
            $closure
        }
        #[cfg(not(target_arch = "wasm32"))]
        {}
    }};
}

pub use event;
