// #[derive(Debug, Clone, Copy, thiserror::Error)]
// #[error("pattern failed to match on path")]
// struct PathMatchError;

// struct PathSegment<P>(P)
// where
//     for<'a> &'a P: std::str::pattern::Pattern<'a>;

// impl<P> Handler<Request<Body>, Request<Body>, PathMatchError> for PathSegment<P>
// where
//     for<'a> &'a P: std::str::pattern::Pattern<'a>,
//     P: Send + Sync,
// {
//     type Fut = impl Future<Output = Result<Request<Body>, PathMatchError>> + Send + Sync;

//     fn handle(
//         &mut self,
//         mut input: Request<Body>,
//     ) -> impl Future<Output = Result<Request<Body>, PathMatchError>> + Send + Sync {
//         async move {
//             let uri = input.uri();
//             let Some(path) = uri.path().strip_prefix(&self.0) else {
//                 return Err(PathMatchError);
//             }; //?.parse().unwrap();

//             let path_and_query = format!(
//                 "{path}{}",
//                 uri.query()
//                     .map(|value| format!("?{value}"))
//                     .unwrap_or_default()
//             )
//             .parse()
//             .unwrap();

//             let mut parts = uri.clone().into_parts();
//             parts.path_and_query = Some(path_and_query);

//             *input.uri_mut() = Uri::from_parts(parts).unwrap();
//             Ok(input)
//         }
//     }
// }
