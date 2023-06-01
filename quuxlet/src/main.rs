#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    pattern,
    impl_trait_in_assoc_type
)]

use quux::{prelude::*, server::hyper::Body};
use std::convert::Infallible;

#[derive(Debug, thiserror::Error)]
#[error("this is pointless, give up")]
struct Useless;

#[tokio::main]
async fn main() {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://src/database/data.db")
        .await
        .unwrap();
    println!("serving on http://localhost:3000...");
    handler(|_: Request| async move { Err::<Response, _>(Useless) })
        .or(handler(|_| async move {
            Ok::<_, Infallible>(Response::new(Body::from("Hello World!")))
        }))
        .serve(([127, 0, 0, 1], 3000))
        .await;
    // the trait bound `quux::quux_server::handler::function::Function<[closure@quuxlet\src\main.rs:21:9: 21:21], [async block@quuxlet\src\main.rs:21:22: 21:99], quux::quux_server::hyper::Request<quux::quux_server::hyper::Body>, quux::quux_server::hyper::Response<quux::quux_server::hyper::Body>, std::convert::Infallible>: std::clone::Clone` is not satisfied

    // let hello = || async move { Ok::<_, Infallible>(Response::new(Body::from("Hello World!"))) };

    // (|request| async move { Err(std::io::Error::new(std::io::ErrorKind::NotFound)) }).or(|_| hello);

    // (|request| hello()).serve(([127, 0, 0, 1], 3000)).await;
    // Ok(())
}
