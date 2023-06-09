use crate::internal::prelude::*;
#[cfg_server]
use quux_server::{
    server::{matcher::MatcherHandler, ContextHandler, Matcher, Server},
    Either, Html, IntoResponse,
};

pub trait Component {
    fn render(self) -> impl Item
    where
        Self: Sized;
}

pub trait Routes: Serialize + DeserializeOwned {
    /// Recursively hydrates the dom, starting at the root app component.
    /// Applies a console panic hook for better debugging.
    /// # Errors
    /// - If there is no init script in the dom (`QUUXInitialise`)
    /// - If the init script doesn't have a shadow tree attached
    /// - If the shadow tree is unparseable
    #[cfg_client]
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

    #[cfg_client]
    fn hydrate(self);

    #[cfg_server]
    fn render_to_string<T: Component + Clone + Serialize>(component: T) -> String
    where
        Self: Sized + From<T>,
    {
        let mut tree = component.clone().render();
        tree.insert_id(0);
        let html = tree.to_string();
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

#[cfg_server]
pub trait ServerExt<H, F, M, R>
where
    R: Routes,
    M: ContextHandler,
    H: ContextHandler,
{
    fn component<T>(
        self,
        matcher: M,
    ) -> Server<impl ContextHandler<InnerOutput = Either<H::InnerOutput, Html>>, F, R>
    where
        T: Component + Clone + Serialize + Send + Sync,
        T: From<M::InnerOutput>,
        R: From<T>;
}

#[cfg_server]
impl<H, F, M, R> ServerExt<H, F, M, R> for Server<H, F, R>
where
    H: ContextHandler,
    H::InnerOutput: IntoResponse,
    R: Routes,
    M: ContextHandler,
{
    fn component<T>(
        self,
        matcher: M,
    ) -> Server<impl ContextHandler<InnerOutput = Either<H::InnerOutput, Html>>, F, R>
    where
        T: Component + Clone + Serialize + Send + Sync,
        T: From<M::InnerOutput>,
        R: From<T>,
    {
        self.route(matcher, |props| {
            quux_server::html(R::render_to_string(T::from(props)))
        })
    }
}
