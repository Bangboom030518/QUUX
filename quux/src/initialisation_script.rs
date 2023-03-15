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
pub struct InitialisationScript<T: component::Enum> {
    #[serde(skip)]
    init_script_path: &'static str,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: component::Enum> Component for InitialisationScript<T> {
    type Props = &'static str;
    type ComponentEnum = T;

    fn init(init_script_path: Self::Props) -> Self {
        Self {
            init_script_path,
            _phantom: std::marker::PhantomData,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render(
        &self,
        _: render::Context<Self::ComponentEnum>,
    ) -> render::Output<Self::ComponentEnum> {
        render::Output {
            html: format!(
                "<script type=\"module\" id=\"__quux_init_script__\" data-quux-tree=\"{}\">{};</script>",
                *crate::TREE_INTERPOLATION_ID,
                self.init_script_path,
            ),
            component_node: crate::render::ClientComponentNode {
                component: self.clone().into(),
                render_context: render::Context::default()
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn render(
        &self,
        _: render::Context<Self::ComponentEnum>,
    ) -> render::Output<Self::ComponentEnum> {
        render::Output::new()
    }
}
