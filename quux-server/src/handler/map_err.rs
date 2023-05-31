use super::Handler;
use std::{error::Error, future::Future, marker::PhantomData};

#[derive(Debug, Clone)]
struct MapErr<H, M, E>
where
    H: Handler,
    M: FnMut(H::Error) -> E + Send + Sync,
    E: Error + Send + Sync,
{
    handler: H,
    mapping: M,
    _phantom: PhantomData<E>,
}

impl<M, H, E> Handler for MapErr<H, M, E>
where
    H: Handler,
    M: FnMut(H::Error) -> E + Send + Sync,
    E: Error + Send + Sync,
{
    type Input = H::Input;
    type Output = H::Output;
    type Error = E;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<<H as Handler>::Output, E>> + 'a {
        async move {
            let Self {
                handler, mapping, ..
            } = self;
            handler.handle(input).await.map_err(mapping)
        }
    }
}

pub trait HandlerExt: Handler {
    fn map_err<M, E>(self, mapping: M) -> impl Handler
    where
        M: FnMut(Self::Error) -> E + Send + Sync,
        E: Error + Send + Sync,
        Self: Sized,
    {
        MapErr {
            handler: self,
            mapping,
            _phantom: PhantomData,
        }
    }
}
