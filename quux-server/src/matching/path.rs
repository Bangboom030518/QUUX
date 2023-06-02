use crate::internal::prelude::*;
use std::{str::FromStr, vec::IntoIter};

pub struct Path<H, I>
where
    H: Handler<Input = Context<(I, IntoIter<String>)>>,
{
    handler: H,
}

impl<H, I> Path<H, I>
where
    H: Handler<Input = Context<(I, IntoIter<String>)>, Output = Context<(I, IntoIter<String>)>>,
    I: Send + Sync,
{
    pub fn static_segment(self, segment: &'static str) -> Path<impl Handler<Input = H::Input>, I> {
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
    ) -> Path<impl Handler<Input = H::Input, Output = Context<((I, T), IntoIter<String>)>>, I>
    where
        T: Send + Sync,
    {
        let handler = self
            .handler
            .and_then(handler(|context: H::Input| async move {
                let (previous, mut segments) = context.output;
                let Some(segment) = (&mut segments).next() else {
                    return Err(super::MatchError)
                };
                let new: T = segment.parse().map_err(|_| super::MatchError)?;
                Ok(Context {
                    request: context.request,
                    url: context.url,
                    output: ((previous, new), segments),
                })
            }));
        Path { handler }
    }
}

impl<H, I> Handler for Path<H, I>
where
    H: Handler<Input = Context<(I, IntoIter<String>)>>,
    I: Send + Sync,
{
    type Input = Context<I>;
    type Output = H::Output;
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

            let output = self
                .handler
                .handle(Context {
                    request: input.request,
                    url: input.url,
                    output: (input.output, segments),
                })
                .await
                .map_err(|_| super::MatchError)?;

            Ok(output)
        }
    }
}

pub fn route<O>() -> Path<impl Handler<Input = Context<((), IntoIter<String>)>>, ()> {
    Path {
        handler: handler(|context: Context<_>| async move {
            Ok::<_, Infallible>(Context {
                request: context.request,
                url: context.url,
                output: ((), context.output),
            })
        }),
    }
}
