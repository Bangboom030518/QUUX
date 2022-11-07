use std::collections::HashMap;

pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub type Context = HashMap<String, String>;

pub trait Component {
    type Props;

    fn init(props: Self::Props) -> Self;

    #[cfg(target = "wasm")]
    fn render(&self, context: Context);

    #[cfg(not(target = "wasm"))]
    fn render(&self, context: Context) -> String;
}

pub struct Store<T: std::fmt::Display> {
    value: T,
}

impl<T: std::fmt::Display> Store<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Store<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub fn init_app<T, P>(component: T)
where
    T: Component<Props = P>,
{
}
