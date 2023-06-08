use crate::{internal::prelude::*, IntoResponse};
pub use matcher::{path, Matcher};

mod matcher;

pub trait ContextHandler:
    Handler<Input = Context<()>, Output = Context<Self::InnerOutput>, Error = Context<Self::InnerError>>
{
    type InnerOutput: Send + Sync;
    type InnerError: Send + Sync;
}

impl<T, O, E> ContextHandler for T
where
    T: Handler<Input = Context<()>, Output = Context<O>, Error = Context<E>>,
    O: Send + Sync,
    E: Send + Sync,
{
    type InnerOutput = O;
    type InnerError = E;
}

pub struct Server<H, F>
where
    H: Handler,
{
    handler: H,
    fallback: F,
}

impl<H, F> Server<H, F>
where
    H: ContextHandler,
{
    pub fn new(handler: H, fallback: F) -> Self {
        Self { handler, fallback }
    }

    pub fn route<M, O>(
        self,
        matcher: M,
        mut mapping: impl FnMut(M::InnerOutput) -> O + Send + Sync,
    ) -> Server<
        impl ContextHandler<InnerOutput = Either<H::InnerOutput, O>, InnerError = M::InnerError>,
        F,
    >
    where
        M: ContextHandler,
        O: Send + Sync + IntoResponse,
    {
        let handler = self
            .handler
            .map_err(|context: H::Error| context.with_output(()))
            .or(
                matcher.map(move |Context { request, output }: M::Output| Context {
                    request,
                    output: mapping(output),
                }),
            )
            .map(Into::into);

        Server::new(handler, self.fallback)
    }

    pub fn fallback<R>(
        self,
        mut mapping: impl FnMut(H::Error) -> R + Send + Sync,
    ) -> Server<H, impl FnMut(H::Error) -> Response + Send + Sync>
    where
        R: IntoResponse + Send + Sync,
    {
        Server::new(self.handler, move |context| {
            mapping(context).into_response()
        })
    }
}

impl<H, F> Server<H, F>
where
    H: ContextHandler,
    H::InnerOutput: IntoResponse,
    F: FnMut(H::Error) -> Response + Send + Sync,
{
    pub async fn serve(self, address: impl Into<SocketAddr>) {
        self.handler
            .map(|context| context.output)
            .map_err(self.fallback)
            .serve(address)
            .await;
    }
}

pub fn server() -> Server<impl ContextHandler<InnerOutput = Infallible, InnerError = ()>, ()> {
    Server::new(handler(|context| async move { Err(context) }), ())
}

pub trait Routes: Send + Sync + Clone {
    fn handle(input: Context<()>) -> Response;
}
