#![feature(async_fn_in_trait, return_position_impl_trait_in_trait, pattern)]
use http::{Request, Response, Uri};
use hyper::{
    server::conn::AddrStream,
    service::{make_service_fn, service_fn},
    Body,
};
use std::{convert::Infallible, error::Error, future::Future, net::SocketAddr};

// TODO: connection
trait Handler<I, O, E: Error> {
    fn handle(&mut self, input: I) -> impl Future<Output = Result<O, E>> + Send + Sync;
}

trait Server: Handler<Request<Body>, Response<Body>, Infallible> {
    // TODO: reduce clones?
    async fn serve(self, addr: impl Into<SocketAddr>)
    where
        Self: Sized + Clone + Send + Sync + 'static,
    {
        let server = hyper::Server::bind(&addr.into()).serve(make_service_fn(|_: &AddrStream| {
            let server = self.clone();
            async move {
                // let server = self.clone();
                Ok::<_, Infallible>(service_fn(move |request| {
                    let server = server.clone();
                    async move { server.clone().handle(request).await.map(Into::into) }
                }))
            }
        }));

        server.await.unwrap();
    }
}

impl<T: Handler<Request<Body>, Response<Body>, Infallible>> Server for T {}

impl<F, Fut, I, O, E> Handler<I, O, E> for F
where
    F: FnMut(I) -> Fut,
    Fut: Future<Output = Result<O, E>> + Send + Sync,
    E: Error,
{
    fn handle(&mut self, input: I) -> impl Future<Output = Result<O, E>> + Send + Sync {
        self(input)
    }
}

#[derive(Debug, Clone, Copy, thiserror::Error)]
#[error("pattern failed to match on path")]
struct PathMatchError;

struct PathSegment<P>(P)
where
    for<'a> &'a P: std::str::pattern::Pattern<'a>;

impl<P> Handler<Request<Body>, Request<Body>, PathMatchError> for PathSegment<P>
where
    for<'a> &'a P: std::str::pattern::Pattern<'a>,
    P: Send + Sync,
{
    fn handle(
        &mut self,
        mut input: Request<Body>,
    ) -> impl Future<Output = Result<Request<Body>, PathMatchError>> + Send + Sync {
        async move {
            let uri = input.uri();
            let Some(path) = uri.path().strip_prefix(&self.0) else {
                return Err(PathMatchError);
            }; //?.parse().unwrap();

            let path_and_query = format!(
                "{path}{}",
                uri.query()
                    .map(|value| format!("?{value}"))
                    .unwrap_or_default()
            )
            .parse()
            .unwrap();

            let mut parts = uri.clone().into_parts();
            parts.path_and_query = Some(path_and_query);

            *input.uri_mut() = Uri::from_parts(parts).unwrap();
            Ok(input)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://src/database/data.db")
        .await
        .unwrap();

    (|request| async move { Ok::<_, Infallible>(Response::new(Body::from("Hello World!"))) })
        .serve(([127, 0, 0, 1], 3000))
        .await;

    Ok(())
}
