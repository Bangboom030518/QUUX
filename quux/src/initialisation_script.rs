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

impl<T: component::Enum + From<Self>> Component<T> for InitialisationScript {
    #[server]
    type Props = &'static str;
    // type ComponentEnum = T;

    #[server]
    fn init(init_script: Self::Props) -> Self {
        Self {
            init_script,
            // _phantom: std::marker::PhantomData,
        }
    }

    fn render(self, context: render::Context<T>) -> render::Output<Self, T> {
        use crate as quux;
        view! {
            context,
            T,
            script("type"="module", id="__quux_init_script__", data-quux-tree = *crate::TREE_INTERPOLATION_ID) {
                {self.init_script}
            }
        }
    }
}
