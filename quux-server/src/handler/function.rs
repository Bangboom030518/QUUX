use super::Handler;
use std::{future::Future, marker::PhantomData, sync::Arc};

#[derive(Clone)]
pub struct Function<F, Fut, I, O, E>
where
    F: FnMut(I) -> Fut,
    Fut: Future<Output = Result<O, E>> + Send + Sync,
{
    handler: F,
    _phantom: PhantomData<Arc<(Fut, I, O, E)>>,
}

// impl<F, Fut, I, O, E> Function<F, Fut, I, O, E>
// where
//     F: FnMut(I) -> Fut + Send + Sync,
//     Fut: Future<Output = Result<O, E>> + Send + Sync,
//     E: Error + Send + Sync,
//     O: Send + Sync,
//     I: Send + Sync,
// {
//     pub fn new(f: F) -> Self {
//         Self {
//             handler: f,
//             _phantom: PhantomData,
//         }
//     }
// }

pub fn handler<F, Fut, I, O, E>(f: F) -> Function<F, Fut, I, O, E>
where
    F: FnMut(I) -> Fut + Send + Sync,
    Fut: Future<Output = Result<O, E>> + Send + Sync,
    E: Send + Sync,
    O: Send + Sync,
    I: Send + Sync,
{
    Function {
        handler: f,
        _phantom: PhantomData,
    }
}

impl<F, Fut, I, O, E> Handler for Function<F, Fut, I, O, E>
where
    F: FnMut(I) -> Fut + Send + Sync,
    Fut: Future<Output = Result<O, E>> + Send + Sync,
    E: Send + Sync,
    O: Send + Sync,
    I: Send + Sync,
{
    type Input = I;
    type Output = O;
    type Error = E;

    #[allow(clippy::needless_lifetimes)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<O, E>> + Send + Sync + 'a {
        (self.handler)(input)
    }
}
