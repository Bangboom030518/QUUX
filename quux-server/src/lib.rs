#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    pattern,
    impl_trait_projections,
    exact_size_is_empty,
    type_alias_impl_trait
)]
pub use hyper;
pub use server::{server, Server};
use std::convert::Infallible;
pub use url::Url;

pub mod handler;
mod matching;
pub mod server;

pub type Request = http::Request<hyper::Body>;
pub type Response = http::Response<hyper::Body>;

#[derive(Debug, thiserror::Error)]
pub enum Either<A, B> {
    #[error(transparent)]
    A(A),
    #[error(transparent)]
    B(B),
}

impl<T> Either<T, T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::A(a) => a,
            Self::B(b) => b,
        }
    }
}

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Ok(value) => value.into_response(),
            Err(value) => value.into_response(),
        }
    }
}

impl<A, B> IntoResponse for Either<A, B>
where
    A: IntoResponse,
    B: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Either::A(value) => value.into_response(),
            Either::B(value) => value.into_response(),
        }
    }
}

impl IntoResponse for Infallible {
    fn into_response(self) -> Response {
        match self {}
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

pub trait ThreadSafe: Send + Sync + Clone {}

impl<T: Send + Sync + Clone> ThreadSafe for T {}

mod internal {
    pub mod prelude {
        pub use super::super::{prelude::*, Either};
        pub use hyper::{
            server::conn::AddrStream,
            service::{make_service_fn, service_fn},
            Body,
        };
        pub use std::{
            convert::Infallible, error::Error, future::Future, marker::PhantomData, net::SocketAddr,
        };
        pub use url::Url;
    }
}

pub mod prelude {
    pub use super::{handler::prelude::*, server, Request, Response, Server, ThreadSafe};
}
