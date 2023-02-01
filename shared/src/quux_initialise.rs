use serde::{Deserialize, Serialize};
use crate::{Component, RenderContext, RenderData, ClientComponentNode, ComponentEnum};


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
pub struct QUUXInitialise {
    #[serde(skip)]
    init_script_content: &'static str,
}

impl Component for QUUXInitialise {
    type Props = Props;

    fn init(
        Self::Props {
            init_script_content,
        }: Self::Props,
    ) -> Self {
        Self {
            init_script_content,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn render<T>(&self, _: RenderContext<T>) -> RenderData<T>
    where
        T: ComponentEnum,
    {
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
    fn render<T>(&self, _: RenderContext<T>) -> RenderData<T>
    where
        T: ComponentEnum,
    {
        RenderData::new()
    }
}
