#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    pattern,
    impl_trait_in_assoc_type
)]

use http::Method;
use quux::{prelude::*, server::Either};
use quuxlet::pages::{Discover, Index};

#[tokio::main]
async fn main() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://src/database/data.db")
        .await
        .unwrap();

    println!("serving on http://localhost:3000...");

    use quux::component::ServerExt;

    // let discover =
    //     .and(with_pool(pool.clone()))
    //     .and_then(|pool| async move { Discover::new(&pool).await.map_err(warp::reject::custom) });
    server::<quuxlet::Routes>()
        .component::<Index>(path(Method::GET))
        .component::<Discover>(
            path(Method::GET)
                .static_segment("discover")
                .and_then(handler(|context: Context<()>| async move {
                    let discover = Discover::new(&pool)
                        .await
                        .map_err(move |err| context.with_output(path::Error::Fatal(err)))?;
                    Ok(context.with_output(discover))
                }))
                .map_err(|error| match error {
                    Either::A(error) => todo!(),
                    Either::B(error) => todo!(),
                }),
        )
        // .route(path(Method::GET), |context| html("HELLO WORLD!"))
        // .route(matcher(Method::POST))
        // .component::<Index>(matching!(path = "hello" / String, method = Get, body = ))
        // .component::<Create>()
        .fallback(|_| quux_server::html("Hello World"))
        .serve(([127, 0, 0, 1], 3000))
        .await;
}
