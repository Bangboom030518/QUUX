use crate::internal::prelude::*;
use std::marker::PhantomData;

pub struct Server<R: Routes> {
    // handler: H,
    _phantom: PhantomData<R>,
}

impl<R: Routes> Server<R> {
    fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    // pub fn route<C, M>(self, matching: crate::Matching) -> Self
    // where
    //     R: From<C>,
    //     // M: Handler<Input = Context<()>>,
    // {
    // }
}

pub fn server<R>() -> Server<R>
where
    R: Routes,
{
    Server::new()
}

impl<R: Routes> Handler for Server<R> {
    type Input = Context<()>;
    type Output = Response;
    type Error = std::convert::Infallible;

    #[allow(clippy::needless_lifetimes, clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
        async move { Ok(R::handle(input)) }
    }
}

pub trait Routes: Send + Sync {
    fn handle(input: Context<()>) -> Response;
}
