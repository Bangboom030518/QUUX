use crate::{ClientComponentNode, Component, ComponentEnum, RenderContext, RenderData};
use serde::{Deserialize, Serialize};

pub struct Props {
    pub init_script_content: &'static str,
}

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
///             @QUUXInitialise
///         }
///     }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct QUUXInitialise<T: ComponentEnum> {
    #[serde(skip)]
    init_script_content: &'static str,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: ComponentEnum> Component for QUUXInitialise<T> {
    type Props = Props;
    type ComponentEnum = T;

    fn init(
        Self::Props {
            init_script_content,
        }: Self::Props,
    ) -> Self {
        Self {
            init_script_content,
            _phantom: std::marker::PhantomData,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render(&self, _: RenderContext<Self::ComponentEnum>) -> RenderData<Self::ComponentEnum> {
        RenderData {
            html: format!(
                "<script type=\"module\" id=\"__quux_init_script__\" data-quux-tree=\"{}\">{};</script>",
                *crate::TREE_INTERPOLATION_ID,
                self.init_script_content,
            ),
            component_node: ClientComponentNode {
                component: self.clone().into(),
                render_context: RenderContext::default()
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn render(&self, _: RenderContext<Self::ComponentEnum>) -> RenderData<Self::ComponentEnum> {
        RenderData::new()
    }
}
