use super::Head;
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Error {
    Timeout,
    Internal,
    PageNotFound(String),
    SetNotFound,
}

impl Error {
    #[server]
    fn title(&self) -> String {
        let title = match self {
            Self::Internal => "Unexpected Error",
            Self::Timeout => "Request Timeout",
            Self::PageNotFound(_) => "Page Not Found",
            Self::SetNotFound => "Set Not Found",
        };
        format!("{title} - QUUXLET")
    }
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Component for Error {
    fn render(self, context: Context<Self>) -> Output<Self> {
        type Component = Error;
        view! {
            context,
            html(lang="en") {
                @Head(self.title())
                body {
                    main {
                        match &self {
                            Self::Internal => {
                                h1 {{ "Internal Server Error!" }}
                            },
                            Self::Timeout => {
                                h1 {{ "Request Timeout!" }}
                            },
                            Self::PageNotFound(uri) => {
                                h1 {{ format!("Page '{uri}' not found!") }}
                            },
                            Self::SetNotFound => {
                                h1 {{ "Page not found!" }}
                            }
                        }
                    }
                    @InitialisationScript(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}

#[server]
impl component::Init for Error {
    type Props = tower::BoxError;

    fn init(error: Self::Props) -> Self {
        if error.is::<tower::timeout::error::Elapsed>() {
            Self::Timeout
        } else {
            Self::Internal
        }
    }
}

#[server]
impl From<Error> for axum::http::StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::Internal => Self::INTERNAL_SERVER_ERROR,
            Error::Timeout => Self::REQUEST_TIMEOUT,
            Error::PageNotFound(_) | Error::SetNotFound => Self::NOT_FOUND,
        }
    }
}
