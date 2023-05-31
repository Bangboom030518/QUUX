// use crate::internal::prelude::*;

// fn or<'future, 'handler, H1, H2, I, O, E1, E2>(
//     base: H1,
//     fallback: H2,
// ) -> impl Handler<'future, E1, O, E2>
// where
//     H1: Handler<'future, I, O, E1> + 'handler,
//     H2: Handler<'future, E1, O, E2> + 'handler,
//     <H1 as Handler<'future, I, O, E1>>::Fut: 'future,
//     <H2 as Handler<'future, E1, O, E1>>::Fut: 'future,
//     E1: Error,
//     E2: Error,
// {
//     |input| async move {
//         match base.handle(input).await {
//             Ok(output) => output,
//             Err(err) => fallback.handle(err).await,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub struct Or<'future, 'handler, H1, H2, I, O, E1, E2>
// where
//     H1: Handler<'future, I, O, E1> + 'handler,
//     H2: Handler<'future, E1, O, E2> + 'handler,
//     <H1 as Handler<'future, I, O, E1>>::Fut: 'future,
//     <H2 as Handler<'future, E1, O, E1>>::Fut: 'future,
//     E1: Error,
//     E2: Error,
// {
//     base: H1,
//     fallback: H2,
//     _phantom: PhantomData<&'future &'handler (I, O, E1, E2)>,
// }

// impl<'future, 'handler, H1, H2, I, O, E1, E2> Handler<'future, I, O, E2>
//     for Or<'future, 'handler, H1, H2, I, O, E1, E2>
// where
//     H1: Handler<'future, I, O, E1> + 'handler,
//     H2: Handler<'future, E1, O, E2> + 'handler,
//     <H1 as Handler<'future, I, O, E1>>::Fut: 'future,
//     <H2 as Handler<'future, E1, O, E1>>::Fut: 'future,
//     E1: Error,
//     E2: Error,
// {
//     type Fut = impl Future<Output = Result<O, E2>> + Send + Sync + 'future;

//     fn handle(&'future mut self, input: I) -> Self::Fut {
//         async move {
//             match self.base.handle(input).await {
//                 Ok(result) => Ok(result),
//                 Err(err) => self.fallback.handle(err).await,
//             }
//         }
//     }
// }

// pub trait HandlerExt<'future, 'handler, I, O, E1>: Handler<'future, I, O, E1>
// where
//     E1: Error,
// {
//     fn or<H2, E2>(self, fallback: H) -> impl Handler<'future, I, O, E2> + 'future
//     where
//         Self: Handler<'future, I, O, E1> + 'handler,
//         H2: Handler<'future, E1, O, E2> + 'handler,
//         <Self as Handler<'future, I, O, E1>>::Fut: 'future,
//         <H2 as Handler<'future, E1, O, E1>>::Fut: 'future,
//         E2: Error,
//     {
//         Or {
//             base: self,
//             fallback,
//             _phantom: PhantomData,
//         }
//     }
// }

// #[test]
// fn or_handler() {
//     #[derive(Debug, thiserror::Error)]
//     #[error("test error")]
//     struct Error;

//     let handler = HandlerExt::or(
//         |input| async move { Err::<Infallible, _>(Error) },
//         |err| async move { Ok::<_, Infallible>("Hello World!") },
//     );
//     assert_eq!(handler.handle("Hi!"), Ok("Hello World"))
// }
