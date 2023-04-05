use crate::internal::prelude::*;

pub trait MapInternal<T, E: Debug> {
    /// Returns a result, mapping the error variant of an `Option` or `Result` to an `errors::Internal` error with `action`.
    /// # Errors
    /// This function should return an error variant if `self` is also the error variant
    fn map_internal_error(self, action: &str) -> Result<T, Internal<E>>;

    /// Maps the error variant of an `Option` or `Result` to an `errors::Internal` error with `action`, unwrapping the result.
    /// # Panics
    /// This function panics on an error variant of `self`, by calling `.unwrap()`.
    fn expect_internal(self, action: &str) -> T
    where
        Self: Sized,
    {
        self.map_internal_error(action).unwrap()
    }
}

impl<T, E: Debug> MapInternal<T, E> for Result<T, E> {
    fn map_internal_error(self, action: &str) -> Result<T, Internal<E>> {
        self.map_err(|error| Internal::new(error, action))
    }
}

impl<T> MapInternal<T, ()> for Option<T> {
    fn map_internal_error(self, action: &str) -> Result<T, Internal<()>> {
        self.map_or_else(|| Err(Internal::new((), action)), Ok)
    }
}

pub struct Internal<T: Debug> {
    error: T,
    action: String,
}

impl<T: Debug> Internal<T> {
    fn new(error: T, action: &str) -> Self {
        Self {
            error,
            action: action.to_string(),
        }
    }
}

impl<T: Debug> Display for Internal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to {} (quux internal error)", self.action)
    }
}

impl<T: Debug> Debug for Internal<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to {} (quux internal error): {:?}",
            self.action, self.error
        )
    }
}

impl<T: Debug> std::error::Error for Internal<T> {}

#[derive(Debug)]
pub enum ClientParse {
    Base64Decode(base64::DecodeError),
    PostcardDecode(postcard::Error),
}

impl Display for ClientParse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Base64Decode(_) => write!(f, "Failed to decode data as base64"),
            Self::PostcardDecode(_) => write!(f, "Failed to decode bytes"),
        }
    }
}

impl std::error::Error for ClientParse {}

#[derive(Debug)]
pub enum InitApp {
    NoInitScript,
    NoTreeOnInitScript,
    InvalidTree(ClientParse),
}

impl Display for InitApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoInitScript => write!(f, "Couldn't find init script in dom. Try adding the `QUUXInitialise` component at the end of the body."),
            Self::NoTreeOnInitScript => write!(f, "No `data-quux-tree` attribute on init script."),
            Self::InvalidTree(error) => write!(f, "Failed to parse tree: '{error}'.")
        }
    }
}

impl std::error::Error for InitApp {}
