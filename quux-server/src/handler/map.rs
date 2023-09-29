use crate::internal::prelude::*;

#[derive(Debug)]
struct Map<H, M, Fut>
where
    H: Handler,
    M: FnMut(H::Output) -> Fut + Send + Sync,
    Fut: Future + Send + Sync,
    Fut::Output: Send + Sync,
{
    handler: H,
    mapping: M,
    // _phantom: PhantomData<Fut>,
}

impl<M, H, Fut> Handler for Map<H, M, Fut>
where
    H: Handler,
    M: FnMut(H::Output) -> Fut + Send + Sync,
    Fut: Future + Send + Sync,
    Fut::Output: Send + Sync,
{
    type Input = H::Input;
    type Output = Fut::Output;
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
            match handler.handle(input).await.map(mapping) {
                Ok(output) => Ok(output.await),
                Err(err) => Err(err),
            }
        }
    }
}

// TODO: combine

pub trait HandlerExt: Handler {
    fn map_async<M, Fut>(
        self,
        mapping: M,
    ) -> impl Handler<Input = Self::Input, Output = Fut::Output, Error = Self::Error>
    where
        M: FnMut(Self::Output) -> Fut + Send + Sync,
        Fut: Future + Send + Sync,
        Fut::Output: Send + Sync,
        Self: Sized,
    {
        Map {
            handler: self,
            mapping,
        }
    }

    fn map<M, O>(
        self,
        mut mapping: M,
    ) -> impl Handler<Input = Self::Input, Output = O, Error = Self::Error>
    where
        M: FnMut(Self::Output) -> O + Send + Sync,
        O: Send + Sync,
        Self: Sized,
    {
        Map {
            handler: self,
            mapping: move |output| std::future::ready(mapping(output)),
        }
    }
}

impl<T: Handler> HandlerExt for T {}
