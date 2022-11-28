use html::view;
use shared::QUUXInitialise;
use shared::{init_app, Component, Render, RenderData, Store};
use warp::Filter;

mod tests;

struct App<'a> {
    count: Store<'a, u32>,
}

impl<'a> Component for App<'a> {
    type Props = ();

    fn init(_props: Self::Props) -> Self {
        Self {
            count: Store::new(0),
        }
    }
}

impl<'a> Render for App<'a> {
    fn render(&self) -> RenderData {
        view! {
            html(lang="en") {
                head {}
                body {
                    button {
                        { self.count }
                    }
                    @QUUXInitialise
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let html = init_app(App::init(()));
    // let html: &str = html.as_str();
    let index = warp::path::end().map(|| html);

    // https://github.com/seanmonstar/warp/blob/master/examples/routing.rs

    warp::serve(warp::get().and(
        index
            .or(warp::fs::file("./dist/wasm/quux_bg.wasm")
    )))
        .run(([127, 0, 0, 1], 3030))
        .await;

}
