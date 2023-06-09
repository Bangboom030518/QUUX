use crate::{internal::prelude::*, IntoResponse};
pub use matcher::{path, Matcher};

pub mod matcher;

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

    pub fn route<M, O>(
        self,
        matcher: M,
        mut mapping: impl FnMut(M::InnerOutput) -> O + Send + Sync,
    ) -> Server<
        impl ContextHandler<InnerOutput = Either<H::InnerOutput, O>, InnerError = M::InnerError>,
        F,
        R,
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
    Server::new(handler(|context| async move { Err(context) }), ())
}

// pub trait Routes: Send + Sync {}
