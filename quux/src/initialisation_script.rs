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
pub struct InitialisationScript {
    #[serde(skip)]
    init_script: &'static str,
    // _phantom: std::marker::PhantomData<T>,
}

#[server]
impl component::Init for InitialisationScript {
    type Props = &'static str;

    fn init(init_script: Self::Props) -> Self {
        Self { init_script }
    }
}

#[typetag::serde]
impl Component for InitialisationScript {
    #[server]
    fn render(self, context: render::Context) -> render::Output<Self> {
        // use crate as quux;

        render::Output::new(
            todo!(),
            render::ClientComponentNode {
                component: Box::new(self),
                render_context: render::Context::default(),
            },
        )
        // view! {
        //     context,
        //     script("type"="module", id="__quux_init_script__", data-quux-tree = *crate::TREE_INTERPOLATION_ID) {
        //         {self.init_script}
        //     }
        // }
    }

    #[client]
    fn render(self, _: render::Context) -> render::Output<Self> {
        render::Output::new(self)
    }
}
