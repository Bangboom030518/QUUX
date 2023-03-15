use crate::internal::prelude::*;

pub trait Component: Serialize + DeserializeOwned {
    type Props;
    type ComponentEnum: Enum;

    fn init(props: Self::Props) -> Self;

    #[cfg(not(target_arch = "wasm32"))]
    fn render_to_string(self) -> String {
        let render::Output {
            html,
            component_node,
        } = self.render(render::Context::default());
        let bytes =
            postcard::to_stdvec(&component_node).expect_internal("serialize `RenderContext`");
        let component_node = base64::encode(bytes);
        format!(
            "<!DOCTYPE html>{}",
            html.replace(&crate::TREE_INTERPOLATION_ID.to_string(), &component_node)
        )
    }

    fn render(self, context: render::Context<Self::ComponentEnum>) -> render::Output<Self>;
}

#[cfg(not(target_arch = "wasm32"))]
pub struct EnumRenderOutput<T>
where
    T: Enum,
{
    pub html: String,
    pub component_node: super::render::ClientComponentNode<T>,
}

#[cfg(target_arch = "wasm32")]
pub struct EnumRenderOutput<T>(std::marker::PhantomData<T>)
where
    T: Enum;

impl<T: Component> From<render::Output<T>> for EnumRenderOutput<<T as Component>::ComponentEnum> {
    #[cfg(not(target_arch = "wasm32"))]
    fn from(value: render::Output<T>) -> Self {
        let render::Output {
            html,
            component_node,
            ..
        } = value;
        Self {
            html,
            component_node,
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn from(_: render::Output<T>) -> Self {
        Self(std::marker::PhantomData)
    }
}

#[cfg(target_arch = "wasm32")]

impl<T: Component> SerializePostcard for T {}

pub trait Enum: Serialize + Debug + Clone + From<InitialisationScript<Self>> {
    fn render(self, context: render::Context<Self>) -> EnumRenderOutput<Self>;

    /// Recursively hydrates the dom, starting at the root app component
    /// Applies a console panic hook for better debugging
    /// # Errors
    /// - If there is no init script in the dom (`QUUXInitialise`)
    /// - If the init script doesn't have a virtual dom tree attached
    /// - If the virtual dom tree is unparseable
    #[cfg(target_arch = "wasm32")]
    fn init_app() -> Result<EnumRenderOutput<Self>, errors::InitApp>
    where
        Self: DeserializeOwned,
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let init_script = crate::dom::document()
            .get_element_by_id("__quux_init_script__")
            .map_or_else(|| Err(errors::InitApp::NoInitScript), Ok)?;

        let tree = init_script
            .get_attribute("data-quux-tree")
            .map_or_else(|| Err(errors::InitApp::NoTreeOnInitScript), Ok)?;

        let tree: render::ClientComponentNode<Self> =
            tree.parse().map_err(errors::InitApp::InvalidTree)?;

        Ok(tree.component.render(tree.render_context))
    }
}
