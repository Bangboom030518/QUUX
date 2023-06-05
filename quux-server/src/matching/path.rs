use crate::internal::prelude::*;
use std::{str::FromStr, vec::IntoIter};

#[derive(Debug, thiserror::Error)]
pub enum PathMatchError<E> {
    Static,
    Dynamic(E),
}

pub type Context<I> = crate::handler::Context<(I, IntoIter<String>)>;

pub trait PathHandler:
    Handler<
    Input = Context<Self::InnerInput>,
    Output = Context<Self::InnerOutput>,
    Error = crate::handler::Context<PathMatchError<Self::InnerError>>,
>
{
    type InnerInput: Send + Sync;
    type InnerOutput: Send + Sync;
    type InnerError: Send + Sync;
}

impl<T, I, O, E> PathHandler for T
where
    T: Handler<
        Input = Context<I>,
        Output = Context<O>,
        Error = crate::handler::Context<PathMatchError<E>>,
    >,
    I: Send + Sync,
    O: Send + Sync,
    E: Send + Sync,
{
    type InnerInput = I;
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
    pub fn static_segment(
        self,
        segment: &'static str,
    ) -> Path<
        impl PathHandler<
            InnerInput = H::InnerInput,
            InnerOutput = H::InnerOutput,
            InnerError = H::InnerError,
        >,
    > {
        Path {
            handler: self
                .handler
                .and_then(handler(
                    move |mut context: Context<H::InnerOutput>| async move {
                        let segments = &mut context.output.1;
                        if segments
                            .next()
                            .is_some_and(|current_segment| current_segment == segment)
                        {
                            Ok(context)
                        } else {
                            Err(context.with_output(PathMatchError::<H::InnerError>::Static))
                        }
                    },
                ))
                .map_err(|err| err.unwrap()),
        }
    }

    pub fn dynamic_segment<T: FromStr>(
        self,
    ) -> Path<
        impl PathHandler<
            InnerInput = H::InnerInput,
            InnerOutput = (H::InnerOutput, T),
            InnerError = Either<H::InnerError, T::Err>,
        >,
    >
    where
        T: Send + Sync,
        T::Err: Send + Sync,
    {
        let handler = self
            .handler
            .map_err(|err| err.map(|err| match err {
                PathMatchError::Static => PathMatchError::Static,
                PathMatchError::Dynamic(err) => PathMatchError::Dynamic(Either::A(err))
            }))
            .and_then(handler::<_, _, H::Output, _, _>(|mut context| async move {
                let segments = &mut context.output.1;
                let Some(segment) = segments.next() else {
                    return Err(context.with_output(PathMatchError::<Either<H::InnerError, T::Err>>::Static))
                };
                let new: T = match segment.parse() {
                    Ok(new) => new,
                    Err(err) => return Err(context.with_output(PathMatchError::Dynamic(Either::B(err))))
                };
                Ok(context.map(|(previous, segments)| ((previous, new), segments)))
            })).map_err(|err| err.unwrap());
        Path { handler }
    }
}

impl<H> Handler for Path<H>
where
    H: PathHandler,
{
    type Input = crate::handler::Context<H::InnerInput>;
    type Output = crate::handler::Context<H::InnerOutput>;
    type Error = crate::handler::Context<PathMatchError<H::InnerError>>;

    // TODO: path args parse failure?
    // TODO: use slice::Split
    #[allow(clippy::needless_lifetimes, clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
        async move {
            let Some(segments) = input.url().path_segments() else {
                return Err(input.with_output(PathMatchError::Static));
            };

            // TODO: .collect().into_iter()
            let segments = segments
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .into_iter();

            let context = input.map(move |output| (output, segments));

            self.handler.handle(context).await.and_then(|context| {
                if !context.output.1.is_empty() {
                    return Err(context.with_output(PathMatchError::Static));
                }
                Ok(context.map(|(output, _)| output))
            })
        }
    }
}

pub fn path<I>() -> Path<impl PathHandler<InnerInput = I, InnerOutput = I, InnerError = Infallible>>
where
    I: Send + Sync,
{
    Path {
        handler: handler(|context: Context<_>| async move { Ok(context) }),
    }
}

#[tokio::test]
async fn path_works() {
    let mut handler = path().static_segment("hello").dynamic_segment::<u32>();

    let request = hyper::Request::builder()
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
        .uri("http://localhost:3000/hello/NaN")
        .body(Body::empty())
        .unwrap();

    let response = handler.handle(crate::handler::Context::new(request)).await;

    assert!(response.is_err());
}
