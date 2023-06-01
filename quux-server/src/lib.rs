#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    pattern,
    impl_trait_in_assoc_type
)]

use std::convert::Infallible;

pub use hyper;
pub use url::Url;

pub mod handler;

pub type Request = http::Request<hyper::Body>;
pub type Response = http::Response<hyper::Body>;

pub(crate) fn expect_uri(url: &Url) -> http::Uri {
    url.as_str()
        .parse()
        .expect("a parsed Url should always be a valid Uri")
}

#[derive(Debug, thiserror::Error)]
pub enum Either<A, B> {
    #[error(transparent)]
    A(A),
    #[error(transparent)]
    B(B),
}

trait IntoResponse {
    fn into_response(self) -> Response;
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
    pub use super::{handler::prelude::*, Request, Response};
}
