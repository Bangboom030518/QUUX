#![feature(more_qualified_paths, stmt_expr_attributes)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]

use quux::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub mod data;
pub mod pages;

routes!(
    pages::Set,
    pages::Error,
    pages::Create,
    pages::Index,
    pages::Discover
);

// struct RouteHandler<R: component::Routes>();

// impl<R: component::Routes> Handler for RouteHandler<R> {
//     fn handle<'a>(
//         &'a mut self,
//         input: Self::Input,
//     ) -> impl std::future::Future<Output = Result<Self::Output, Self::Error>> + Send + Sync + 'a
//     {
//     }
// }

/// # Panics
/// This function will panic if it's unable to retrieve and parse the tree sent by the server
// #[cfg(target_arch = "wasm32")]
#[cfg_client]
#[wasm_bindgen(start)]
pub fn init_app() {
    Routes::init_app().unwrap();
}
