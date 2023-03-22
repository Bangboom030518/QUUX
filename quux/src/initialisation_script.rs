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

#[server]
impl component::Init for InitialisationScript {
    type Props = &'static str;

    fn init(init_script: Self::Props) -> Self {
        Self { init_script }
    }
}

impl Component for InitialisationScript {
    fn render(self, context: crate::view::Context<Self>) -> crate::view::Output<Self> {
        use crate as quux;

        type Component = InitialisationScript;
        view! {
            context,
            script("type"="module", id="__quux_init_script__", data-quux-tree = *crate::TREE_INTERPOLATION_ID) {
                {self.init_script}
            }
        }
    }
}
