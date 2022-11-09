use std::collections::HashMap;
pub use stores::Store;

pub mod stores;

pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub type Context = HashMap<String, String>;

pub fn init_app<T, P>(component: T)
where
    T: Component<Props = P>,
{
}

pub struct RenderData {
    pub html: String,
}

pub trait Component {
    type Props;

    fn init(props: Self::Props) -> Self;

    #[cfg(target = "wasm")]
    fn render(&self, context: Context);

    #[cfg(not(target = "wasm"))]
    fn render(&self) -> RenderData;
}
