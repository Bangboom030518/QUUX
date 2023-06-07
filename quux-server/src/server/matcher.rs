use crate::internal::prelude::*;
use std::{str::FromStr, vec::IntoIter};

#[derive(Debug, thiserror::Error)]
pub enum MatchError<E> {
    Static,
    Dynamic(E),
    IncorrectMethod,
}

pub type Context<I> = crate::handler::Context<(I, IntoIter<String>)>;

pub trait MatcherHandler:
    Handler<
    Input = Context<Self::InnerInput>,
    Output = Context<Self::InnerOutput>,
    Error = crate::handler::Context<MatchError<Self::InnerError>>,
>
{
    type InnerInput: Send + Sync;
    type InnerOutput: Send + Sync;
    type InnerError: Send + Sync;
}

impl<T, I, O, E> MatcherHandler for T
where
    T: Handler<
        Input = Context<I>,
        Output = Context<O>,
        Error = crate::handler::Context<MatchError<E>>,
    >,
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync,
{
    type InnerInput = I;
    type InnerOutput = O;
    type InnerError = E;
}

pub struct Matcher<H>
where
    H: MatcherHandler,
{
    handler: H,
}

impl<H> Matcher<H>
where
    H: MatcherHandler,
{
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    /// Ensures that path has a segment of the string passed
    pub fn static_segment(
        self,
        segment: &'static str,
    ) -> Matcher<
        impl MatcherHandler<
            InnerInput = H::InnerInput,
            InnerOutput = H::InnerOutput,
            InnerError = H::InnerError,
        >,
    > {
        Matcher::new(
            self.handler
                .and_then(handler(move |mut context: H::Output| async move {
                    let segments = &mut context.output.1;
                    if segments
                        .next()
                        .is_some_and(|current_segment| current_segment == segment)
                    {
                        Ok(context)
                    } else {
                        Err(context.with_output(MatchError::<H::InnerError>::Static))
                    }
                }))
                .map_err(|err| err.unwrap()),
        )
    }

    // Parses a path segment
    pub fn dynamic_segment<T: FromStr>(
        self,
    ) -> Matcher<
        impl MatcherHandler<
            InnerInput = H::InnerInput,
            InnerOutput = (H::InnerOutput, T),
            InnerError = Either<H::InnerError, T::Err>,
        >,
    >
    where
        T: Send + Sync,
        T::Err: Send + Sync,
    {
        Matcher::new(self
            .handler
            .map_err(|err| err.map(|err| match err {
                MatchError::Dynamic(err) => MatchError::Dynamic(Either::A(err)),
                MatchError::Static => MatchError::Static,
                MatchError::IncorrectMethod => MatchError::IncorrectMethod,
            }))
            .and_then(handler(|mut context: H::Output| async move {
                let segments = &mut context.output.1;
                let Some(segment) = segments.next() else {
                    return Err(context.with_output(MatchError::<Either<H::InnerError, T::Err>>::Static))
                };
                let new: T = match segment.parse() {
                    Ok(new) => new,
                    Err(err) => return Err(context.with_output(MatchError::Dynamic(Either::B(err))))
                };
                Ok(context.map(|(previous, segments)| ((previous, new), segments)))
            })).map_err(|err| err.unwrap()))
    }
}

impl<H> Handler for Matcher<H>
where
    H: MatcherHandler,
{
    type Input = crate::handler::Context<H::InnerInput>;
    type Output = crate::handler::Context<H::InnerOutput>;
    type Error = crate::handler::Context<MatchError<H::InnerError>>;

    // TODO: path args parse failure?
    // TODO: use slice::Split
    #[allow(clippy::needless_lifetimes, clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
        async move {
            let segments = input.url().path_segments();

            // TODO: .collect().into_iter()
            let segments = segments
                .into_iter()
                .flatten()
                .peekable()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .into_iter();

            let context = input.map(move |output| (output, segments));

            self.handler.handle(context).await.and_then(|mut context| {
                let segments = &mut context.output.1;
                if !segments.is_empty() && segments.next() != Some(String::new()) {
                    return Err(context.with_output(MatchError::Static));
                }
                Ok(context.map(|(output, _)| output))
            })
        }
    }
}

pub fn path<I>(
    method: http::Method,
) -> Matcher<impl MatcherHandler<InnerInput = I, InnerOutput = I, InnerError = Infallible>>
where
    I: Send + Sync,
{
    Matcher::new(handler(move |context: Context<_>| {
        let result = if context.request.method() == method {
            Ok(context)
        } else {
            Err(context.with_output(MatchError::IncorrectMethod))
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
        .uri("http://localhost:3000")
        .body(Body::empty())
        .unwrap();

    let response = handler.handle(crate::handler::Context::new(request)).await;

    assert!(response.is_ok());
}
