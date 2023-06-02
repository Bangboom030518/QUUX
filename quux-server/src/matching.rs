use crate::internal::prelude::*;

mod path;

#[derive(Debug, thiserror::Error)]
#[error("failed to match in handler")]
pub struct MatchError;

pub struct Matching<P> {
    _phantom: PhantomData<P>,
}

// impl<P> Matching<P> {
//     pub fn route(route: Route) -> Self {}
// }

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

pub fn matching<P>() -> Matching<P> {
    Matching {
        _phantom: PhantomData,
    }
}
