# The Server Component of QUUX

```rust
#[cfg(ignore)]
fn a() {
    server()
        .route(matching!(Method::Get, "/"), |request| -> Response {
            handler()
        })
        .route(path!("path1"), |request| -> Response { handler() })
        .route(path!("path2"), |request| -> Response { handler() })
        .or_default(|request| -> Response { handler() })
        .error(|error| -> Response { handler() });
    // route = (Request) -> Option<Result<Response, E>>

    // generic handler = (I) -> (I, Result<O, E>)
    // and(
    //     handler: (I) -> (I, Result<O1, E1>),
    //     and: (I, O) -> (I, Result<O2, E2>)
    // ) = (I) -> (I, Result<O2, Either<E1, E2>>)
    fn and(handler: !, and: !) -> ! {
        |input| {
            let (input, result) = handler(input);
            let output = result.map_err(Either::A)?;
            let (input, result) = and(input, output);
            (input, result.map_err(Either::A))
        }
    }

    // (Request) -> (Request, Result<(), FailedMatch>)
    get()
        // (Request) -> (Request, Result<u8, Either<FailedMatch, FailedMatch>>)
        .and(path!(u8))
        // (Request) -> (Request, Result<Response, Either<Either<FailedMatch, FailedMatch>>, DatabaseError>>)
        .map(|request, (number)| async move { Response::new(query_database(request)) })
}
```
