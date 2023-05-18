#![warn(clippy::pedantic, clippy::nursery)]
// TODO: remove?
#![allow(clippy::unused_async)]
#![cfg(not(target_arch = "wasm32"))]
use std::{collections::HashMap, convert::Infallible};

use http::Uri;
use quuxlet::pages::{create, error, Create, Error, Index, Set};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use warp::{path::FullPath, Filter};

fn with_pool(
    pool: Pool<Sqlite>,
) -> impl Filter<Extract = (Pool<Sqlite>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

#[tokio::main]
async fn main() {
    const PORT: u16 = 3000;

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://src/database/data.db")
        .await
        .unwrap();

    let index = warp::path::end().and(warp::get()).map(|| Index);

    let set = with_pool(pool)
        .and(warp::path!("set" / String))
        .and(warp::get())
        .and_then(|pool, id: String| async move {
            Set::new(&pool, &id).await.map_err(warp::reject::custom)
        });

    let not_found = warp::path::full().and_then(|path: FullPath| async move {
        Err::<Infallible, _>(warp::reject::custom(error::NotFound(
            path.as_str().parse().unwrap(),
        )))
    });

    println!("listening on http://localhost:{PORT}");

    warp::serve(
        index
            .or(set)
            .or(Create::routes())
            .or(warp::path!("dist" / "quuxlet_bg.wasm")
                .and(warp::filters::fs::file("./dist/quuxlet_bg.wasm"))
                .with(warp::reply::with::header(
                    "Content-Type",
                    "application/wasm",
                )))
            .or(not_found)
            .recover(|rejection| async move {
                Ok::<_, std::convert::Infallible>(Error::from(rejection))
            }),
    )
    .run(([127, 0, 0, 1], PORT))
    .await;
}
