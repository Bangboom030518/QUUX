use quux::{Component, init_app, Context, Store};

struct App {
    count: Store<u32>,
}

impl Component for App {
    type Props = ();

    fn init(props: Self::Props) -> Self {
        Self {
            count: Store::new(0)
        }
    }

    fn render(&self, context: Context) -> String {
        format!("<button id='random'>{}</button>", self.count)
    }
}

fn main() {
    init_app(App::init(()));
}
