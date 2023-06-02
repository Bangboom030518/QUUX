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

fn expect_url(uri: &Uri) -> Url {
    uri.to_string()
        .parse()
        .expect("a parsed Uri should always be a valid Url")
}

pub struct Context<O> {
    pub(crate) request: crate::Request,
    pub(crate) url: Url,
    pub output: O,
}

impl Context<()> {
    fn new(request: crate::Request) -> Self {
        Self {
            url: expect_url(request.uri()),
            request,
            output: (),
        }
    }
}

impl<O> Context<O> {
    pub fn url(&self) -> &Url {
        &self.url
    }

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
    type Error: Send + Sync;

    #[allow(clippy::needless_lifetimes)]
    #[allow(clippy::manual_async_fn)]
    fn handle<'a>(
        &'a mut self,
        input: Self::Input,
    ) -> impl Future<Output = Result<Self::Output, Self::Error>> + Send + Sync + 'a;

    async fn serve(self, addr: impl Into<SocketAddr>)
    where
        Self: Sized + Handler<Input = Context<()>> + 'static,
        Result<Self::Output, Self::Error>: IntoResponse,
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
        map_err::HandlerExt as _, or::HandlerExt as _, Context, Handler,
    };
}
