#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    pattern,
    impl_trait_in_assoc_type
)]

use http::Method;
use quux::server::prelude::*;
use std::convert::Infallible;

#[tokio::main]

async fn main() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://src/database/data.db")
        .await
        .unwrap();

    println!("serving on http://localhost:3000...");

    // handler(|request: Context<()>| async move {
    //     let path = request.url().path();
    //     if path == "/" {
    //         Ok(Response::new(Body::from("Hello World!")))
    //     } else {
    //         Err(request)
    //     }
    // })
    // .or(handler(|request: Context<()>| async move {
    //     Ok::<_, Infallible>(Response::new(Body::from(format!(
    //         "Hello {}!",
    //         request.url()
    //     ))))
    // }))
    // .serve(([127, 0, 0, 1], 3000))
    // .await;

    // matching().path().method(Method::Get).body::<>;

    server()
        .route(
            path(Method::GET).map(|context| {
                dbg!(context.request().uri());
                context
            }),
            |context| html("HELLO WORLD!"),
        )
        // .route(matcher(Method::POST))
        // .component::<Index>(matching!(path = "hello" / String, method = Get, body = ))
        // .component::<Create>()
        .fallback(|_| html("HELLO!"))
        .serve(([127, 0, 0, 1], 3000))
        .await;
}
