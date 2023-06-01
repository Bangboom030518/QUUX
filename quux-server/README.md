# The Server Component of QUUX

```rust
server()
    .route(matching!(Method::Get, "/" + String), |request| -> Response {
        handler()
    })
    .route(matching!(...), Component::new())

    .route(path!("path1"), |request| -> Response { handler() })
    .route(path!("path2"), |request| -> Response { handler() })
    .or_default(|request| -> Response { handler() })
    .error(|error| -> Response { handler() });

struct Context<T> {
    request: Request,
    output: T,
    url: Url,
}

any()
    .or(
        any()
            // Error = Either<Infallible, MatchFailure>
            // -> = Result<Context<Request>, Context<MatchFailure>>
            .and_then(|context| if context.method == Method::Get {
                Ok(context)
            } else {
                Err(context.data(MatchFailure))
            })
            // Error = Either<Either<Infallible, MatchFailure>, MatchFailure>
            .and_then(|context| context
                .match_static_path_segment("hello")
                .match_path_segment::<String>()
                .and_then(|string| string)
            )
            // Error = Either<Either<Either<Infallible, MatchFailure>, MatchFailure>, DatabaseError>
            .and_then(|context| {
                let name = query_database(context)?;
                Ok(context.data(Response::new(format!("Hello {name}"))))
            })
            FnMut<Args = (A, B, C), Output = ()>
    )
    .or(
        any()
            .and_then(match_route("api"))
            .and_then(
                {
                    let handler =
                        match_route("number")
                            .and_then(respond(42))
                            .or(
                                any()
                                    .and_then(match_route("number"))
                                    .and_then(respond(42))
                            );
                    |context| handler.handle(context)
                }
            )
    )
    .or(|err| {

    })

// (Context<()>) -> Result<Context<()>, Context<FailedMatch>>
get()
    // (Context<()>) -> Result<Context<u8>, Context<Either<FailedMatch, FailedMatch>>>
    .and_then(path!(u8))
    // (Context<u8>) -> Result<Context<Response>, Context<Either<Either<FailedMatch, FailedMatch>>, DatabaseError>>>
    .and_then(|context| async move { context.data(Response::new(query_database(context.request, context.extracted))) })

```
