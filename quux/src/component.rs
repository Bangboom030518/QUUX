use crate::{
    internal::prelude::*,
    render::{ClientComponentNode, Context},
};

#[typetag::serde(tag = "type")]
pub trait Component {
    #[server]
    fn render_to_string(self) -> String
    where
        Self: Serialize + Sized,
    {
        let render::Output {
            html,
            component_node,
            ..
        } = self.render(render::Context::default());
        let bytes =
            postcard::to_stdvec(&component_node).expect_internal("serialize `RenderContext`");
        let component_node = base64::encode(bytes);
        format!(
            "<!DOCTYPE html>{}",
            html.replace(&crate::TREE_INTERPOLATION_ID.to_string(), &component_node)
        )
    }

    fn render(self, context: render::Context) -> render::Output<Self>
    where
        Self: Sized;
}

pub trait Init {
    type Props;
    fn init(props: Self::Props) -> Self;
}

#[client]
impl<E, T> SerializePostcard for T
where
    E: Enum + From<T>,
    T: Component<E>,
{
}
