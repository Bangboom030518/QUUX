use html::view;
use shared::Store;

mod tests;

// struct App {
//     count: Store<u32>,
// }

// impl Component for App {
//     type Props = ();

//     fn init(props: Self::Props) -> Self {
//         Self {
//             count: Store::new(0)
//         }
//     }

//     fn render(&self) -> RenderData {
//         format!("<button id='random'>{}</button>", self.count)
//     }
// }

fn main() {
    view! {
        body {
            h1(a="hello!") {
                { hello }
            }
            makka {
                { "abc" }
            }
        }
    };
    let mut my_juicy_store = Store::new(0);
    my_juicy_store
        .on_change(|previous, current| println!("{} will change to {}, juicy!", previous, current));

    my_juicy_store.on_change(|_, current| {
        println!("I found a {}", current);
    });

    my_juicy_store.on_change(|previous, _| {
        println!("It was {}", previous);
    });

    for _ in 0..3 {
        my_juicy_store.set(my_juicy_store.get() + 1);
    }
    // init_app(App::init(()));
}
