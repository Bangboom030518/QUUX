use crate::internal::prelude::*;

pub trait Component {
    fn render(self, context: crate::context::Context<Self>) -> impl Item
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
        tree.hydrate();
        Ok(())
    }

    #[client]
    fn hydrate(self);

    #[server]
    fn render_to_string<T: Component + Clone + Serialize>(component: T) -> String
    where
        Self: Sized + From<T>,
    {
        let mut tree = component.clone().render(crate::context::Context::new());
        tree.insert_id(0);
        let html = tree.to_string();
        // TODO: serialize component
        let component = Self::from(component);
        let bytes = postcard::to_stdvec(&component).expect_internal("serialize `RenderContext`");
        let tree = base64::encode(bytes);
        format!(
            "<!DOCTYPE html>{}",
            html.replace("$$QUUX_TREE_INTERPOLATION$$", &tree)
        )
    }
}

impl<T: Routes> SerializePostcard for T {}
