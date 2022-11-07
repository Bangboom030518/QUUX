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

pub fn init_app(component: dyn Component) {

}
