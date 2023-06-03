use crate::internal::prelude::*;
use std::{str::FromStr, vec::IntoIter};

pub type Context<I> = crate::handler::Context<(I, IntoIter<String>)>;

pub struct Path<H, I, O>
where
    H: Handler<Input = Context<I>, Output = Context<O>>,
{
    handler: H,
}

impl<H, I, O> Path<H, I, O>
where
    H: Handler<Input = Context<I>, Output = Context<O>>,
    O: Send + Sync,
    I: Send + Sync,
{
    pub fn static_segment(
        self,
        segment: &'static str,
    ) -> Path<impl Handler<Input = Context<I>, Output = Context<O>>, I, O> {
        Path {
            handler: self
                .handler
                .and_then(handler(move |context: H::Output| async move {
                    let mut segments = context.output.1;
                    if segments
                        .next()
                        .is_some_and(|current_segment| current_segment == segment)
                    {
                        Ok(Context {
                            request: context.request,
                            url: context.url,
                            output: (context.output.0, segments),
                        })
                    } else {
                        Err(super::MatchError)
                    }
                })),
        }
    }

    pub fn dynamic_segment<T: FromStr>(
        self,
    ) -> Path<impl Handler<Input = Context<I>, Output = Context<(O, T)>>, I, (O, T)>
    where
        T: Send + Sync,
    {
        let handler = self
            .handler
            .and_then(handler(|context: H::Input| async move {
                let (previous, mut segments) = context.output;
                let Some(segment) = segments.next() else {
                    return Err(super::MatchError)
                };
                let new: T = segment.parse().map_err(|_| super::MatchError)?;
                Ok(Context {
                    request: context.request,
                    url: context.url,
                    output: ((context.output.0, new), segments),
                })
            }));
        Path { handler }
    }
}

impl<H, O, I> Handler for Path<H, I, O>
where
    H: Handler<Input = Context<I>, Output = Context<O>>,
    O: Send + Sync,
    I: Send + Sync,
{
    type Input = crate::handler::Context<I>;
    type Output = crate::handler::Context<O>;
    type Error = super::MatchError;

    // TODO: path args parse failure?
    // TODO: use slice::Split
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
        async move {
            let Some(segments) = input.url().path_segments() else {
                return Err(super::MatchError);
            };

            // TODO: .collect().into_iter()
            let segments = segments
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .into_iter();

            self.handler
                .handle(crate::handler::Context {
                    request: input.request,
                    url: input.url,
                    output: (input.output, segments),
                })
                .await
                .map_err(|_| super::MatchError)
                .and_then(|context| {
                    if !context.output.1.is_empty() {
                        return Err(super::MatchError);
                    }
                    Ok(crate::handler::Context {
                        request: context.request,
                        url: context.url,
                        output: context.output.0,
                    })
                })
        }
    }
}

pub fn path<I>() -> Path<impl Handler<Input = Context<I>, Output = Context<I>>, I, I>
where
    I: Send + Sync,
{
    Path {
        handler: handler(|context: Context<_>| async move {
            Ok::<_, Infallible>(Context {
                request: context.request,
                url: context.url,
                output: context.output,
            })
        }),
    }
}
