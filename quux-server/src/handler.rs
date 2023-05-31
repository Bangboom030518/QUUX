use http::Request;
use hyper::Body;
use std::{error::Error, future::Future, marker::PhantomData};
use url::Url;

pub mod and_then;
pub mod map_err;
pub mod or;
pub mod path_segment;
pub mod server;

// TODO: consider
struct Context<O> {
    request: Request<Body>,
    url: Url,
    output: O,
}

#[derive(Debug, thiserror::Error)]
enum Either<A, B> {
    #[error(transparent)]
    A(A),
    #[error(transparent)]
    B(B),
}

// TODO: connection
pub trait Handler: Send + Sync {
    type Input: Send + Sync;
    type Output: Send + Sync;
    type Error: Error + Send + Sync;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + Sync + 'a;
}

// pub trait IntoHandler {
//     fn into_handler<'a>(self) -> impl Handler<'a>;
// }

// struct HandlerFn<F, Fut, I, O, E>
// where
//     F: FnMut(I) -> Fut,
//     Fut: Future<Output = Result<O, E>> + Send + Sync,
//     E: Error,
// {
//     handler: F,
//     _phantom: PhantomData<(Fut, I, O, E)>,
// }

// impl<'a, F, I, O, E> Handler<'a> for HandlerFn<F, Fut, I, O, E>
// where
//     F: FnMut(I) -> Fut,
//     // Fut: Future<Output = Result<O, E>> + Send + Sync,
//     E: Error,
// {
//     type Input = I;
//     type Output = O;
//     type Error = E;

//     fn handle(&mut self, input: Self::Input) -> impl Future<Output = Result<O, E>> + Send + Sync {
//         self(input)
//     }
// }
