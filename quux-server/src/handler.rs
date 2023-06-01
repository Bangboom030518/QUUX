use crate::internal::prelude::*;
use std::{error::Error, future::Future, sync::Arc};
use url::Url;

pub mod and_then;
pub mod any;
pub mod function;
pub mod map;
pub mod map_err;
pub mod or;
pub mod path_segment;

pub struct Context<O> {
    request: Request,
    url: Url,
    output: O,
}

impl<O> Context<O> {
    pub fn with_output<T>(self, output: T) -> Context<T> {
        let Self { request, url, .. } = self;
        Context {
            request,
            url,
            output,
        }
    }
}

// TODO: connection
pub trait Handler: Send + Sync {
    type Input: Send + Sync;
    type Output: Send + Sync;
    type Error: Error + Send + Sync;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + Sync + 'a;

    async fn serve(self, addr: impl Into<SocketAddr>)
    where
        Self: Sized + Handler<Input = Request, Output = Response> + 'static,
    {
        // TODO: Mutex means we lose the benfit of async
        let server = Arc::new(tokio::sync::Mutex::new(self));
        let server =
            hyper::Server::bind(&addr.into()).serve(make_service_fn(move |_: &AddrStream| {
                let server = Arc::clone(&server);
                async move {
                    let server = Arc::clone(&server);
                    Ok::<_, Infallible>(service_fn(move |request| {
                        let server = Arc::clone(&server);
                        async move { server.lock().await.handle(request).await.map(Into::into) }
                    }))
                }
            }));

        server.await.unwrap();
    }
}

pub mod prelude {
    pub use super::{
        and_then::HandlerExt as _, function::handler, map::HandlerExt as _,
        map_err::HandlerExt as _, or::HandlerExt as _, Handler,
    };
}
