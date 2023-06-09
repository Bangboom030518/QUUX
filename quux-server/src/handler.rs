use crate::{internal::prelude::*, IntoResponse};
use http::Uri;
use std::{future::Future, sync::Arc};
use url::Url;

pub mod and_then;
pub mod any;
pub mod function;
pub mod map;
pub mod map_err;
pub mod or;
pub mod then;

#[derive(Debug)]
pub struct Context<O> {
    pub(crate) request: crate::Request,
    pub output: O,
}

// impl<O: Clone> Clone for Context<O> {
//     fn clone(&self) -> Self {
//         let Self {
//             request,
//             url,
//             output,
//         } = &self;
//         let builder = hyper::Request::builder()
//             .method(request.method().clone())
//             .uri(request.uri().clone())
//             .version(request.version().clone())
//             .extension(request.extensions());
//         for (key, value) in request.headers() {
//             builder.header(key, value);
//         }
//         let request = builder.body(*request.body()).unwrap();
//         Self {
//             url: url.clone(),
//             output: output.clone(),
//             request,
//         }
//     }
// }

impl Context<()> {
    pub fn new(request: crate::Request) -> Self {
        Self {
            request,
            output: (),
        }
    }
}

impl<O> Context<O> {
    pub fn request(&self) -> &Request {
        &self.request
    }

    pub fn with_output<T>(self, output: T) -> Context<T> {
        let Self { request, .. } = self;
        Context { request, output }
    }

    pub fn map<T>(self, mapping: impl FnOnce(O) -> T) -> Context<T> {
        let Self { request, output } = self;
        Context {
            request,
            output: mapping(output),
        }
    }
}

// TODO: connection
pub trait Handler: Send + Sync {
    type Input: Send + Sync;
    type Output: Send + Sync;
    type Error: Send + Sync;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + Sync + 'a;

    async fn serve(self, address: impl Into<SocketAddr>)
    where
        Self: Sized + Handler<Input = Context<()>> + 'static,
        Result<Self::Output, Self::Error>: IntoResponse,
    {
        // TODO: Mutex means we lose the benfit of async
        let server = Arc::new(tokio::sync::Mutex::new(self));
        let server =
            hyper::Server::bind(&address.into()).serve(make_service_fn(move |_: &AddrStream| {
                let server = Arc::clone(&server);
                async move {
                    let server = Arc::clone(&server);
                    Ok::<_, Infallible>(service_fn(move |request| {
                        let server = Arc::clone(&server);
                        async move {
                            Ok::<_, Infallible>(
                                server
                                    .lock()
                                    .await
                                    .handle(Context::new(request))
                                    .await
                                    .into_response(),
                            )
                        }
                    }))
                }
            }));

        server.await.unwrap();
    }
}

pub mod prelude {
    pub use super::{
        and_then::HandlerExt as _, function::handler, map::HandlerExt as _,
        map_err::HandlerExt as _, or::HandlerExt as _, then::HandlerExt as _, Context, Handler,
    };
}
