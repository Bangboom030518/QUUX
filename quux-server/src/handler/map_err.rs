use crate::internal::prelude::*;

#[derive(Debug, Clone)]
struct MapErr<H, M, E>
where
    H: Handler,
    M: FnMut(H::Error) -> E + Send + Sync,
    E: Send + Sync,
{
    handler: H,
    mapping: M,
    _phantom: PhantomData<E>,
}

impl<M, H, E> Handler for MapErr<H, M, E>
where
    H: Handler,
    M: FnMut(H::Error) -> E + Send + Sync,
    E: Send + Sync,
{
    type Input = H::Input;
    type Output = H::Output;
    type Error = E;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
        async move {
            let Self {
                handler, mapping, ..
            } = self;
            handler.handle(input).await.map_err(mapping)
        }
    }
}

pub trait HandlerExt: Handler {
    fn map_err<M, E>(
        self,
        mapping: M,
    ) -> impl Handler<Input = Self::Input, Output = Self::Output, Error = E>
    where
        M: FnMut(Self::Error) -> E + Send + Sync,
        E: Send + Sync,
        Self: Sized,
    {
        MapErr {
            handler: self,
            mapping,
            _phantom: PhantomData,
        }
    }
}

impl<T: Handler> HandlerExt for T {}
