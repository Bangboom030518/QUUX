use std::sync::Arc;

use crate::internal::prelude::*;

pub use path::{path, Path};
use tokio::sync::Mutex;

mod path;

#[derive(Debug, Clone, thiserror::Error)]
#[error("failed to match in handler")]
pub struct MatchError;

pub struct Matching<H, O>
where
    H: Handler<Input = Context<()>, Output = Context<O>>,
    O: Send + Sync,
{
    handler: H,
}

impl<H, O> Matching<H, O>
where
    H: Handler<Input = Context<()>, Output = Context<O>>,
    O: Send + Sync,
{
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
    /*
    expected struct `handler::Context<()>`
       found struct `handler::Context<O>`
    */
    pub fn path<H2, O2>(
        self,
        path: Path<H2, O, O2>,
    ) -> Matching<impl Handler<Input = Context<()>, Output = Context<(O, O2)>>, (O, O2)>
    where
        H2: Handler<Input = path::Context<O>, Output = path::Context<O2>>,
        O2: ThreadSafe,
        O: Clone,
    {
        // TODO: `Arc<Mutex<_>>`?
        let path = Arc::new(Mutex::new(path));
        Matching::new(self.handler.and_then(handler({
            let path = Arc::clone(&path);
            move |context: Context<O>| {
                let path = Arc::clone(&path);
                async move {
                    let previous = context.output.clone();
                    let context = path.lock().await.handle(context).await?;
                    let new = context.output.clone();
                    Ok::<_, MatchError>(context.with_output((previous, new)))
                }
            }
        })))
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

pub fn matching() -> Matching<impl Handler<Input = Context<()>, Output = Context<()>>, ()> {
    Matching::new(handler(|context: Context<()>| async move {
        Ok::<_, Infallible>(context)
    }))
}
