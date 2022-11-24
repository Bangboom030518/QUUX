use html::view;
use shared::{Store, Component, Render, RenderData, init_app};

mod tests;

struct App<'a> {
    count: Store<'a, u32>,
}

impl<'a> Component for App<'a> {
    type Props = ();

    fn init(_props: Self::Props) -> Self {
        Self {
            count: Store::new(0)
        }
    }

}

impl<'a> Render for App<'a> {
    fn render(&self) -> RenderData {
        view! {
            button {
                { self.count }
            }
        }
    }
}

fn main() {
    init_app(App::init(()));
}
