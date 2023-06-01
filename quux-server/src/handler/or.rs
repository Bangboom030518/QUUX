use crate::internal::prelude::*;

#[derive(Debug, Clone)]
struct Or<H1, H2>
where
    H1: Handler,
    H2: Handler<Input = H1::Error>,
{
    base: H1,
    fallback: H2,
}

impl<H1, H2> Handler for Or<H1, H2>
where
    H1: Handler,
    H2: Handler<Input = H1::Error>,
{
    type Input = H1::Input;
    type Output = Either<H1::Output, H2::Output>;
    type Error = H2::Error;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
        async move {
            match self.base.handle(input).await {
                Ok(output) => Ok(Either::A(output)),
                Err(err) => self.fallback.handle(err).await.map(Either::B),
            }
        }
    }
}

pub trait HandlerExt: Handler {
    fn or<H>(self, fallback: H) -> impl Handler
    where
        H: Handler<Input = Self::Error>,
        Self: Sized,
    {
        Or {
            base: self,
            fallback,
        }
    }
}

impl<T: Handler> HandlerExt for T {}
