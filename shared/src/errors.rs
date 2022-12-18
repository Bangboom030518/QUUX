pub trait MapInternal<T, E: std::fmt::Debug> {
    /// Returns a result, mapping the error variant of an `Option` or `Result` to an `errors::Internal` error with `action`.
    /// # Errors
    /// This function should return an error variant if `self` is also the error variant
    fn map_internal_error(self, action: &'static str) -> Result<T, Internal<E>>;

    /// Maps the error variant of an `Option` or `Result` to an `errors::Internal` error with `action`, unwrapping the result.
    /// # Panics
    /// This function panics on an error variant of `self`, by calling `.unwrap()`.
    fn expect_internal(self, action: &'static str) -> T
    where
        Self: Sized,
    {
        self.map_internal_error(action).unwrap()
    }
}

impl<T, E: std::fmt::Debug> MapInternal<T, E> for Result<T, E> {
    fn map_internal_error(self, action: &'static str) -> Result<T, Internal<E>> {
        self.map_err(|error| Internal::new(error, action))
    }
}

impl<T> MapInternal<T, ()> for Option<T> {
    fn map_internal_error(self, action: &'static str) -> Result<T, Internal<()>> {
        self.map_or_else(|| Err(Internal::new((), action)), Ok)
    }
}

pub struct Internal<T: std::fmt::Debug> {
    error: T,
    action: &'static str,
}

impl<T: std::fmt::Debug> Internal<T> {
    const fn new(error: T, action: &'static str) -> Self {
        Self { error, action }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for Internal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to {} (quux internal error)", self.action)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Internal<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Failed to {} (quux internal error): {:?}",
            self.action, self.error
        )
    }
}

impl<T: std::fmt::Debug> std::error::Error for Internal<T> {}

pub enum ClientParse {
    Base64Decode(base64::DecodeError),
    PostcardDecode(postcard::Error),
}

impl ClientParse {
    const BASE_64_MESSAGE: &str = "Failed to decode data as base64";
    const POSTCARD_DECODE_MESSAGE: &str = "Failed to decode bytes";
}

impl std::fmt::Display for ClientParse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base64Decode(_) => write!(f, "{}", Self::BASE_64_MESSAGE),
            Self::PostcardDecode(_) => write!(f, "{}", Self::POSTCARD_DECODE_MESSAGE),
        }
    }
}

impl std::fmt::Debug for ClientParse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Base64Decode(err) => write!(f, "{}: {err:?}", Self::BASE_64_MESSAGE),
            Self::PostcardDecode(err) => write!(f, "{}: {err:?}", Self::POSTCARD_DECODE_MESSAGE),
        }
    }
}

impl std::error::Error for ClientParse {}
