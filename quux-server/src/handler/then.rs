use crate::internal::prelude::*;

#[derive(Debug)]
struct Then<H1, H2>
where
    H1: Handler,
    H2: Handler<Input = Result<H1::Output, H1::Error>>,
{
    base: H1,
    then: H2,
}

impl<H1, H2> Handler for Then<H1, H2>
where
    H1: Handler,
    H2: Handler<Input = Result<H1::Output, H1::Error>>,
{
    type Input = H1::Input;
    type Output = H2::Output;
    type Error = H2::Error;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
        async move { self.then.handle(self.base.handle(input).await).await }
    }
}

pub trait HandlerExt: Handler {
    fn then<H>(
        self,
        then: H,
    ) -> impl Handler<Input = Self::Input, Output = H::Output, Error = H::Error>
    where
        H: Handler<Input = Result<Self::Output, Self::Error>>,
        Self: Sized,
    {
        Then { base: self, then }
    }
}

impl<T: Handler> HandlerExt for T {}
