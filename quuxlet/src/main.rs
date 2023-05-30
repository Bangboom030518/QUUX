#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    pattern,
    impl_trait_in_assoc_type
)]

use http::{Request, Response};
use quux::{prelude::*, server::hyper::Body};
use std::convert::Infallible;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://src/database/data.db")
        .await
        .unwrap();

    let hello = || async move { Ok::<_, Infallible>(Response::new(Body::from("Hello World!"))) };

    (|request| async move { Err(std::io::Error::new(std::io::ErrorKind::NotFound)) }).or(|_| hello);

    (|request| hello()).serve(([127, 0, 0, 1], 3000)).await;

    Ok(())
}
