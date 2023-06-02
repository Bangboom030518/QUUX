use crate::internal::prelude::*;

/// Put this in the root component, at the end of the body
///
/// # Example
///
/// ```
/// view! {
///     html {
///         ...
///         body {
///             ...
///             @InitialisationScript
///         }
///     }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InitialisationScript {
    #[serde(skip)]
    init_script: &'static str,
}

impl InitialisationScript {
    pub fn new(init_script: &'static str) -> Self {
        Self { init_script }
    }
}

impl Component for InitialisationScript {
    fn render(self) -> impl Item {
        script()
            .attribute("type", "module")
            .attribute("id", "__quux_init_script__")
            .attribute("data-quux-tree", "$$QUUX_TREE_INTERPOLATION$$")
            .text(self.init_script)
    }
}
