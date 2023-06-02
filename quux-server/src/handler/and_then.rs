use crate::internal::prelude::*;

#[derive(Debug, Clone)]
pub struct AndThen<H1, H2>
where
    H1: Handler,
    H2: Handler<Input = H1::Output>,
{
    base: H1,
    mapping: H2,
}

impl<H1, H2> Handler for AndThen<H1, H2>
where
    H1: Handler,
    H2: Handler<Input = H1::Output>,
{
    type Input = H1::Input;
    type Output = H2::Output;
    type Error = Either<H1::Error, H2::Error>;

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
    fn and_then<H>(self, mapping: H) -> AndThen<Self, H>
    where
        H: Handler<Input = Self::Output>,
        Self: Sized,
    {
        AndThen {
            base: self,
            mapping,
        }
    }
}

impl<T: Handler> HandlerExt for T {}
