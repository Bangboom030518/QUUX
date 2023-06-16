use crate::{internal::prelude::*, IntoResponse};
pub use path::{path, Path};

pub mod path;

pub trait ContextHandler:
    Handler<
    Input = Context<()>,
    Output = Context<Self::InnerOutput>,
    Error = Context<path::Error<Self::InnerError>>,
>
{
    type InnerOutput: Send + Sync;
    type InnerError: Send + Sync;
}

impl<T, O, E> ContextHandler for T
where
    T: Handler<Input = Context<()>, Output = Context<O>, Error = Context<path::Error<E>>>,
    O: Send + Sync,
    E: Send + Sync,
{
    type InnerOutput = O;
    type InnerError = E;
}

pub struct Server<H, F, R>
where
    H: Handler,
{
    handler: H,
    fallback: F,
    _phantom: PhantomData<R>,
}

impl<H, F, R> Server<H, F, R>
where
    H: ContextHandler,
{
    pub fn new(handler: H, fallback: F) -> Self {
        Self {
            handler,
            fallback,
            _phantom: PhantomData,
        }
    }

    pub fn map_handler<M, H2>(self, mapping: M) -> Server<H2, F, R>
    where
        M: FnOnce(H) -> H2,
        H2: ContextHandler,
    {
        Server {
            handler: mapping(self.handler),
            fallback: self.fallback,
            _phantom: PhantomData,
        }
    }

    pub fn route<M, Fut>(
        self,
        matcher: M,
        mapping: impl FnMut(M::InnerOutput) -> Fut + Send + Sync + Clone,
    ) -> Server<
        impl ContextHandler<
            InnerOutput = Either<H::InnerOutput, Fut::Output>,
            InnerError = Either<H::InnerError, M::InnerError>,
        >,
        F,
        R,
    >
    where
        M: ContextHandler,
        Fut: Future + Send + Sync,
        Fut::Output: Send + Sync + IntoResponse,
    {
        // TODO: should mapping take `Context<O>` rather than just `O`
        let handler = handler(move |context: H::Error| async move {
            let Context { request, output } = context;
            match output.fatal() {
                Some(output) => Err(Context { request, output }),
                None => Ok(Context::new(request)),
            }
        })
        .and_then(matcher)
        .map_err(|context| match context {
            Either::A(context) => context.map(|err| path::Error::Fatal(Either::A(err))),
            Either::B(context) => context.map(|err| match err.fatal() {
                Some(err) => path::Error::Fatal(Either::B(err)),
                None => path::Error::PathMatch,
            }),
        })
        .map_async(move |context| {
            let mapping = mapping.clone();
            async move { context.map_async(mapping).await }
        });

        Server::new(
            self.handler.or(handler).map(Context::<Either<_, _>>::from),
            self.fallback,
        )
    }

    pub fn fallback<O>(
        self,
        mut mapping: impl FnMut(H::Error) -> O + Send + Sync,
    ) -> Server<H, impl FnMut(H::Error) -> Response + Send + Sync, R>
    where
        O: IntoResponse + Send + Sync,
    {
        Server::new(self.handler, move |context| {
            mapping(context).into_response()
        })
    }
}

impl<H, F, R> Server<H, F, R>
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

pub fn server<R>() -> Server<impl ContextHandler<InnerOutput = Infallible, InnerError = ()>, (), R>
{
    Server::new(
        handler(
            |context: Context<()>| async move { Err(context.with_output(path::Error::PathMatch)) },
        ),
        (),
    )
}
