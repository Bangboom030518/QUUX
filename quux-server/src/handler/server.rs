// use crate::internal::prelude::*;
// pub trait Server: for<'a> Handler<'a, Request<Body>, Response<Body>, Infallible> {
//     // TODO: reduce clones?
//     async fn serve(self, addr: impl Into<SocketAddr>)
//     where
//         Self: Sized + Clone + Send + Sync + 'static,
//     {
//         let server = hyper::Server::bind(&addr.into()).serve(make_service_fn(|_: &AddrStream| {
//             let server = self.clone();
//             async move {
//                 // let server = self.clone();
//                 Ok::<_, Infallible>(service_fn(move |request| {
//                     let server = server.clone();
//                     async move { server.clone().handle(request).await.map(Into::into) }
//                 }))
//             }
//         }));

//         server.await.unwrap();
//     }
// }

// impl<T> Server for T where T: for<'a> Handler<'a, Request<Body>, Response<Body>, Infallible> {}
