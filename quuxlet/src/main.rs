#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    pattern,
    impl_trait_in_assoc_type
)]

use http::Method;
use quux::prelude::*;
use quuxlet::pages::Index;

#[tokio::main]

async fn main() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://src/database/data.db")
        .await
        .unwrap();

    println!("serving on http://localhost:3000...");

    use quux::component::ServerExt;

    server::<quuxlet::Routes>()
        .component::<Index>(path(Method::GET))
        // .route(path(Method::GET), |context| html("HELLO WORLD!"))
        // .route(matcher(Method::POST))
        // .component::<Index>(matching!(path = "hello" / String, method = Get, body = ))
        // .component::<Create>()
        .fallback(|_| quux_server::html("Hello World"))
        .serve(([127, 0, 0, 1], 3000))
        .await;
}
