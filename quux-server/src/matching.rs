use std::sync::Arc;

use crate::internal::prelude::*;

pub use path::{path, Path};
use tokio::sync::Mutex;

mod path;

#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to match in handler")]
pub struct MatchError;

pub struct Matching<H, I, O>
where
    H: Handler<Input = Context<I>, Output = Context<(I, O)>>,
    I: Send + Sync,
    O: Send + Sync,
{
    handler: H,
}

impl<H, I, O> Matching<H, I, O>
where
    H: Handler<Input = Context<I>, Output = Context<(I, O)>>,
    I: Send + Sync,
    O: Send + Sync,
{
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
    /*
type mismatch resolving `<handler::and_then::AndThen<H, matching::path::Path<H2, (I, O), O2>> as handler::Handler>::Input == handler::Context<(I, O)>`
            */
    pub fn path<H2, O2>(
        self,
        path: Path<H2, (I, O), O2>,
    ) -> Matching<impl Handler<Input = Context<(I, O)>, Output = Context<((I, O), O2)>>, (I, O), O2>
    where
        H2: Handler<Input = path::Context<(I, O)>, Output = path::Context<((I, O), O2)>>,
        O2: ThreadSafe,
        O: Clone,
    {
        /*
                    move |context: Context<O>| {
                let path = Arc::clone(&path);
                async move {
                    let previous = context.output.clone();
                    let context = path.lock().await.handle(context).await?;
                    let new = context.output.clone();
                    Ok::<_, MatchError>(context.with_output((previous, new)))
                }
            }

        */
        Matching::new(self.handler.and_then(path))
    }
}

// impl<P> Handler for Matching<P>
// where
//     P: Send + Sync,
// {
//     type Input = Context<()>;
//     type Output = Context<P>;
//     type Error = Context<MatchError>;

//     fn handle<'a>(
//         &'a mut self,
//         input: Self::Input,
//     ) -> impl Future<Output = Result<Self::Output, Self::Error>> + 'a {
//         async move {}
//     }
// }

pub fn matching<I>() -> Matching<impl Handler<Input = Context<I>, Output = Context<(I, ())>>, I, ()>
where
    I: Send + Sync,
{
    Matching::new(handler(|context: Context<()>| async move {
        Ok::<_, Infallible>(context)
    }))
}
