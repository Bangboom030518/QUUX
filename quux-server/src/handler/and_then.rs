use super::{Either, Handler};
use std::{error::Error, future::Future, marker::PhantomData};

#[derive(Debug, Clone)]
struct AndThen<H1, H2, I, O, O2, E1, E2>
where
    H1: Handler<Input = I, Output = O, Error = E1>,
    H2: Handler<Input = O, Output = O2, Error = E2>,
{
    base: H1,
    mapping: H2,
    _phantom: PhantomData<(I, O, O2, E1, E2)>,
}

impl<H1, H2, I, O, O2, E1, E2> Handler for AndThen<H1, H2, I, O, O2, E1, E2>
where
    H1: Handler<Input = I, Output = O, Error = E1>,
    H2: Handler<Input = O, Output = O2, Error = E2>,
    E1: Error + Send + Sync,
    E2: Error + Send + Sync,
    I: Send + Sync,
    O: Send + Sync,
    O2: Send + Sync,
{
    type Input = I;
    type Output = O2;
    type Error = Either<E1, E2>;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + Sync + 'a {
        async move {
            match self.base.handle(input).await {
                Err(err) => Err(Either::A(err)),
                Ok(output) => self.mapping.handle(output).await.map_err(Either::B),
            }
        }
    }
}

pub trait HandlerExt: Handler {
    fn and_then<H, O, E>(
        self,
        mapping: H,
    ) -> impl Handler<Input = Self::Input, Output = H::Output, Error = Either<Self::Error, H::Error>>
    where
        H: Handler<Input = Self::Output>,
        Self: Sized,
    {
        AndThen {
            base: self,
            mapping,
            _phantom: PhantomData,
        }
    }
}
