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
    // _phantom: std::marker::PhantomData<T>,
}

impl component::Init for InitialisationScript {
    type Props = &'static str;

    fn init(init_script: Self::Props) -> Self {
        Self { init_script }
    }
}

impl Component for InitialisationScript {
    fn render(self, _: crate::context::Context<Self>) -> impl Item {
        script()
            .attribute("type", "module")
            .attribute("id", "__quux_init_script__")
            .attribute("data-quux-tree", "$$QUUX_TREE_INTERPOLATION$$")
    }
}
