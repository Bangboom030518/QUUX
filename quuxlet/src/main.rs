#![warn(clippy::pedantic, clippy::nursery)]
// TODO: remove?
#![allow(clippy::unused_async)]
#![cfg(not(target_arch = "wasm32"))]
use http::Uri;
use quuxlet::pages::{create, error, Create, Discover, Error, Index, Set};
use serde::Deserialize;
use sqlx::{Pool, Sqlite};
use std::{collections::HashMap, convert::Infallible};
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

    let discover = warp::path!("discover")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and_then(|pool| async move { Discover::new(&pool).await.map_err(warp::reject::custom) });

    let set = warp::path!("set" / String)
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and_then(|id: String, pool| async move {
            Set::new(&pool, &id).await.map_err(warp::reject::custom)
        });

    // let not_found = warp::path::full().and_then(|path: FullPath| async move {
    //     Err::<Infallible, _>(warp::reject::custom(error::NotFound(
    //         path.as_str().parse().unwrap(),
    //     )))
    // });

    println!("listening on http://localhost:{PORT}");
    let create = warp::path!("create")
        .and(warp::get())
        .map(|| Create)
        .or(warp::path!("create")
            .and(warp::post())
            .and(with_pool(pool))
            .and(warp::body::form::<create::PostData>())
            .and_then({
                |pool: sqlx::Pool<sqlx::Sqlite>, data: create::PostData| async move {
                    println!("{data:?}");
                    let set = quuxlet::data::Set::create(&pool, &data.name, data.terms)
                        .await
                        .map_err(|error| warp::reject::custom(error::Database::from(error)))?;

                    // TODO: `.parse()` is infallible
                    Ok::<_, warp::Rejection>(warp::redirect(
                        format!("/set/{}", set.id).parse::<http::Uri>().unwrap(),
                    ))
                }
            }));

    warp::serve(
        index
            .or(set)
            .or(create)
            .or(discover)
            .or(warp::path!("dist" / "quuxlet_bg.wasm")
                .and(warp::filters::fs::file("./dist/quuxlet_bg.wasm"))
                .with(warp::reply::with::header(
                    "Content-Type",
                    "application/wasm",
                )))
            // .or(not_found)
            .recover(|rejection| async move {
                eprintln!("Error: {rejection:?}");
                Ok::<Error, std::convert::Infallible>(Error::from(rejection))
            }),
    )
    .run(([127, 0, 0, 1], PORT))
    .await;
}
