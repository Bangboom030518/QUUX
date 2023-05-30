#![allow(incomplete_features)]
#![feature(
    async_fn_in_trait,
    return_position_impl_trait_in_trait,
    pattern,
    impl_trait_in_assoc_type
)]

pub use hyper;
use url::Url;

pub mod handler;

pub fn expect_uri(url: &Url) -> http::Uri {
    url.as_str()
        .parse()
        .expect("a parsed Url should always be a valid Uri")
}

// TODO: connection

mod internal {
    pub mod prelude {
        pub use super::super::prelude::*;
        pub use http::{Request, Response};
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
    pub use super::handler::{or::HandlerExt as _, server::Server, Handler};
}
