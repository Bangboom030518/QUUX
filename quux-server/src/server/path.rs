use crate::internal::prelude::*;
use std::{str::FromStr, vec::IntoIter};

#[derive(Debug, thiserror::Error)]
pub enum Error<E> {
    PathMatch,
    Fatal(E),
}

impl<E> Error<E> {
    pub fn fatal(self) -> Option<E> {
        if let Self::Fatal(error) = self {
            Some(error)
        } else {
            None
        }
    }
}

pub type Context<I> = crate::handler::Context<(I, IntoIter<String>)>;

pub trait PathHandler:
    Handler<
    Input = Context<()>,
    Output = Context<Self::InnerOutput>,
    Error = crate::handler::Context<Error<Self::InnerError>>,
>
{
    type InnerOutput: Send + Sync;
    type InnerError: Send + Sync;
}

impl<T, O, E> PathHandler for T
where
    T: Handler<Input = Context<()>, Output = Context<O>, Error = crate::handler::Context<Error<E>>>,
    O: Send + Sync,
    E: Send + Sync,
{
    type InnerOutput = O;
    type InnerError = E;
}

pub struct Path<H>
where
    H: PathHandler,
{
    handler: H,
}

impl<H> Path<H>
where
    H: PathHandler,
{
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    /// Ensures that path has a segment of the string passed
    pub fn static_segment(
        self,
        segment: &'static str,
    ) -> Path<impl PathHandler<InnerOutput = H::InnerOutput, InnerError = H::InnerError>> {
        Path::new(
            self.handler
                .and_then(handler(move |mut context: H::Output| async move {
                    let segments = &mut context.output.1;
                    if segments
                        .next()
                        .is_some_and(|current_segment| current_segment == segment)
                    {
                        Ok(context)
                    } else {
                        Err(context.with_output(Error::<H::InnerError>::PathMatch))
                    }
                }))
                .map_err(|err| err.unwrap()),
        )
    }

    // Parses a path segment
    pub fn dynamic_segment<T: FromStr>(
        self,
    ) -> Path<
        impl PathHandler<InnerOutput = (H::InnerOutput, T), InnerError = Either<H::InnerError, T::Err>>,
    >
    where
        T: Send + Sync,
        T::Err: Send + Sync,
    {
        Path::new(
            self.handler
                .map_err(|err| {
                    err.map(|err| match err {
                        Error::Fatal(err) => Error::Fatal(Either::A(err)),
                        Error::PathMatch => Error::PathMatch,
                    })
                })
                .and_then(handler(|mut context: H::Output| async move {
                    let segments = &mut context.output.1;
                    let Some(segment) = segments.next() else {
                        return Err(context.with_output(Error::<Either<H::InnerError, T::Err>>::PathMatch))
                    };
                    let new: T = match segment.parse() {
                        Ok(new) => new,
                        Err(err) => return Err(context.with_output(Error::Fatal(Either::B(err)))),
                    };
                    Ok(context.map(|(previous, segments)| ((previous, new), segments)))
                }))
                .map_err(|err| err.unwrap()),
        )
    }
}

impl<H> Handler for Path<H>
where
    H: PathHandler,
{
    type Input = crate::handler::Context<()>;
    type Output = crate::handler::Context<H::InnerOutput>;
    type Error = crate::handler::Context<Error<H::InnerError>>;

    // TODO: path args parse failure?
    // TODO: use slice::Split
    #[allow(clippy::needless_lifetimes, clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
        async move {
            // split the path into segments
            // https://docs.rs/url/latest/src/url/lib.rs.html#1338-1341
            let segments = input
                .request
                .uri()
                .path()
                .strip_prefix('/')
                .map(|remainder| remainder.split('/'));

            // TODO: .collect().into_iter()
            let mut segments = segments
                .into_iter()
                .flatten()
                .map(ToString::to_string)
                .collect::<Vec<String>>();

            // remove segment after trailing slash
            if segments.last() == Some(&String::new()) {
                segments.pop();
            }

            let segments = segments.into_iter();

            let context = input.map(move |output| (output, segments));

            self.handler.handle(context).await.and_then(|mut context| {
                let segments = &mut context.output.1;
                if !segments.is_empty() {
                    return Err(context.with_output(Error::PathMatch));
                }
                Ok(context.map(|(output, _)| output))
            })
        }
    }
}

pub fn path(
    method: http::Method,
) -> Path<impl PathHandler<InnerOutput = (), InnerError = Infallible>> {
    Path::new(handler(move |context: Context<_>| {
        let result = if context.request.method() == method {
            Ok(context)
        } else {
            Err(context.with_output(Error::PathMatch))
        };
        std::future::ready(result)
    }))
}

#[tokio::test]
async fn path_works() {
    let mut handler = path(http::Method::GET)
        .static_segment("hello")
        .dynamic_segment::<u32>();

    let request = hyper::Request::builder()
        .method(http::Method::GET)
        .uri("http://localhost:3000/hello/16")
        .body(Body::empty())
        .unwrap();

    assert_eq!(
        handler
            .handle(crate::handler::Context::new(request))
            .await
            .unwrap()
            .output,
        ((), 16)
    );

    let request = hyper::Request::builder()
        .method(http::Method::GET)
        .uri("http://localhost:3000/hello/NaN")
        .body(Body::empty())
        .unwrap();

    let response = handler.handle(crate::handler::Context::new(request)).await;

    assert!(response.is_err());
}

#[tokio::test]
async fn methods_work() {
    let mut handler = path(http::Method::GET);

    let request = hyper::Request::builder()
        .method(http::Method::POST)
        .uri("http://localhost:3000")
        .body(Body::empty())
        .unwrap();

    let response = handler.handle(crate::handler::Context::new(request)).await;

    assert!(response.is_err());

    let request = hyper::Request::builder()
        .method(http::Method::GET)
        .uri("http://localhost:3000/")
        .body(Body::empty())
        .unwrap();

    let response = handler.handle(crate::handler::Context::new(request)).await;

    assert!(response.is_ok());
}
