use crate::{internal::prelude::*, view::ClientContext};

pub trait Component: Serialize + ClientContext {
    // #[server]
    // fn render_page(self) -> SerializedComponent<Self>
    // //String
    // where
    //     Self: Sized,
    // {
    //     let crate::view::Output {
    //         html,
    //         component_node,
    //         ..
    //     } = self.render(crate::view::Context::new(0, None));
    //     let bytes =
    //         postcard::to_stdvec(&component_node).expect_internal("serialize `RenderContext`");
    //     let component_node = base64::encode(bytes);
    //     format!(
    //         "<!DOCTYPE html>{}",
    //         html.replace(&crate::TREE_INTERPOLATION_ID.to_string(), &component_node)
    //     )
    // }

    fn render(self, context: crate::view::Context<Self>) -> crate::view::Output<Self>
    where
        Self: Sized;
}

pub trait Init {
    type Props;
    fn init(props: Self::Props) -> Self;
}

#[client]
impl<T: Component> SerializePostcard for T {}

/// Recursively hydrates the dom, starting at the root app component
/// Applies a console panic hook for better debugging
/// # Errors
/// - If there is no init script in the dom (`QUUXInitialise`)
/// - If the init script doesn't have a shadow tree attached
/// - If the shadow tree is unparseable

pub trait Routes {
    #[client]
    fn init_app() -> Result<(), errors::InitApp>
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

        let tree: Self = tree.parse().map_err(errors::InitApp::InvalidTree)?;

        Ok(tree.render())
    }

    #[client]
    fn render(self);

    #[server]
    fn render_to_string<T: Component>(component: T) -> String {
        let crate::view::Output {
            html,
            component_node,
            ..
        } = component.render(crate::view::Context::new(0, None));
        let bytes =
            postcard::to_stdvec(&component_node).expect_internal("serialize `RenderContext`");
        let component_node = base64::encode(bytes);
        format!(
            "<!DOCTYPE html>{}",
            html.replace(&crate::TREE_INTERPOLATION_ID.to_string(), &component_node)
        )
    }
}
