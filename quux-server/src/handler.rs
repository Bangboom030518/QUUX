use std::{error::Error, future::Future};

pub mod or;
pub mod path_segment;
pub mod server;

// TODO: connection
pub trait Handler<'a, I, O, E: Error> {
    type Fut: Future<Output = Result<O, E>> + Send + Sync + 'a;

    fn handle(&'a mut self, input: I) -> Self::Fut;
}

impl<'a, F, Fut, I, O, E> Handler<'a, I, O, E> for F
where
    F: FnMut(I) -> Fut,
    Fut: Future<Output = Result<O, E>> + Send + Sync + 'a,
    E: Error,
{
    type Fut = impl Future<Output = Result<O, E>> + Send + Sync + 'a;

    fn handle(&mut self, input: I) -> Self::Fut {
        self(input)
    }
}
