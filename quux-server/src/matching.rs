use crate::{internal::prelude::*, IntoResponse};
pub use path::{path, Path};

mod path;

#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to match in handler")]
pub struct MatchError;

pub trait ContextHandler:
    Handler<Input = Context<()>, Output = Context<Self::InnerOutput>, Error = Context<Self::InnerError>>
{
    type InnerOutput: Send + Sync + IntoResponse;
    type InnerError: Send + Sync;
}

impl<T, O, E> ContextHandler for T
where
    T: Handler<Input = Context<()>, Output = Context<O>, Error = Context<E>>,
    O: Send + Sync + IntoResponse,
    E: Send + Sync,
{
    type InnerOutput = O;
    type InnerError = E;
}

pub struct Matching<H>
where
    H: Handler,
{
    handler: H,
}

impl<H> Matching<H>
where
    H: ContextHandler,
{
    pub fn new(handler: H) -> Self {
        Self { handler }
    }

    pub fn route<M, O>(
        self,
        matcher: M,
        mut mapping: impl FnMut(M::InnerOutput) -> O + Send + Sync,
    ) -> Matching<
        impl ContextHandler<InnerOutput = Either<H::InnerOutput, O>, InnerError = M::InnerError>,
    >
    where
        M: ContextHandler,
        O: Send + Sync + IntoResponse,
    {
        let handler = self
            .handler
            .map_err(|context: H::Error| context.with_output(()))
            .or(matcher.map(
                move |Context {
                          request,
                          url,
                          output,
                      }: M::Output| {
                    Context {
                        request,
                        url,
                        output: mapping(output),
                    }
                },
            ))
            .map(Into::into);
        Matching { handler }
    }

    pub fn handler(
        self,
    ) -> impl Handler<Input = Context<()>, Output = impl IntoResponse, Error = impl IntoResponse>
    {
        self.handler
            .map(|context| context.output)
            .map_err(|error| todo!("add error handler in Self") as Response)
    }
}

pub fn matching() -> Matching<impl ContextHandler<InnerOutput = Infallible, InnerError = ()>> {
    Matching::new(handler(|context| async move { Err(context) }))
}
