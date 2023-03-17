use crate::internal::prelude::*;

pub trait Component: Serialize {
    type ClientContext: Serialize + DeserializeOwned;

    #[server]
    fn render_to_string(self) -> String
    where
        Self: Sized,
    {
        let crate::view::Output {
            html,
            component_node,
            ..
        } = self.render(crate::view::Context::new(0, None));
        let bytes =
            postcard::to_stdvec(&component_node).expect_internal("serialize `RenderContext`");
        let component_node = base64::encode(bytes);
        format!(
            "<!DOCTYPE html>{}",
            html.replace(&crate::TREE_INTERPOLATION_ID.to_string(), &component_node)
        )
    }

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
