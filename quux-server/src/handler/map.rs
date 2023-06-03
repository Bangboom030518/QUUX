use crate::internal::prelude::*;

#[derive(Debug, Clone)]
struct Map<H, M, O>
where
    H: Handler,
    M: FnMut(H::Output) -> O + Send + Sync,
    O: Send + Sync,
{
    handler: H,
    mapping: M,
    _phantom: PhantomData<O>,
}

impl<M, H, O> Handler for Map<H, M, O>
where
    H: Handler,
    M: FnMut(H::Output) -> O + Send + Sync + Clone,
    O: Send + Sync + Clone,
{
    type Input = H::Input;
    type Output = O;
    type Error = H::Error;

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
            handler.handle(input).await.map(mapping)
        }
    }
}

pub trait HandlerExt: Handler {
    fn map<M, O>(self, mapping: M) -> impl Handler
    where
        M: FnMut(Self::Output) -> O + Send + Sync + Clone,
        O: Send + Sync + Clone,
        Self: Sized,
    {
        Map {
            handler: self,
            mapping,
            _phantom: PhantomData,
        }
    }
}

impl<T: Handler> HandlerExt for T {}
