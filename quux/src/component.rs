use crate::internal::prelude::*;

pub trait Component: Serialize + ComponentChildren {
    fn render(self, context: crate::view::Context<Self>) -> crate::view::Output<Self>
    where
        Self: Sized;
}

pub trait Init {
    type Props;
    fn init(props: Self::Props) -> Self;
}

pub trait Routes: Serialize + DeserializeOwned {
    /// Recursively hydrates the dom, starting at the root app component.
    /// Applies a console panic hook for better debugging.
    /// # Errors
    /// - If there is no init script in the dom (`QUUXInitialise`)
    /// - If the init script doesn't have a shadow tree attached
    /// - If the shadow tree is unparseable
    #[client]
    fn init_app() -> Result<(), errors::InitApp> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let init_script = crate::dom::document()
            .get_element_by_id("__quux_init_script__")
            .map_or_else(|| Err(errors::InitApp::NoInitScript), Ok)?;

        let tree = init_script
            .get_attribute("data-quux-tree")
            .map_or_else(|| Err(errors::InitApp::NoTreeOnInitScript), Ok)?;

        let tree = Self::deserialize_base64(&tree).map_err(errors::InitApp::InvalidTree)?;
        tree.render();
        Ok(())
    }

    #[client]
    fn render(self);

    #[server]
    fn render_to_string<T: Component>(component: T) -> String
    where
        Self: Sized + From<SerializedComponent<T>>,
    {
        let crate::view::Output {
            html,
            component_node,
            ..
        } = component.render(crate::view::Context::new(0, None));
        let component_node = Self::from(component_node);
        let bytes =
            postcard::to_stdvec(&component_node).expect_internal("serialize `RenderContext`");
        let component_node = base64::encode(bytes);
        format!(
            "<!DOCTYPE html>{}",
            html.replace("$$QUUX_TREE_INTERPOLATION$$", &component_node)
        )
    }
}

impl<T: Routes> SerializePostcard for T {}
