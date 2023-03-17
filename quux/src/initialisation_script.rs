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

#[derive(Serialize, Deserialize, Default)]
pub struct ClientContext {
    components: (),
}

impl Component for InitialisationScript {
    type ClientContext = ClientContext;

    #[server]
    fn render(self, context: crate::view::Context<Self>) -> crate::view::Output<Self> {
        // use crate as quux;

        crate::view::Output::new(
            todo!(),
            crate::view::SerializedComponent {
                component: self,
                render_context: crate::view::Context::new(0, None),
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
    fn render(self, _: crate::view::Context) -> crate::view::Output<Self> {
        crate::view::Output::new(self)
    }
}
