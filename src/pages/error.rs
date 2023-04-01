use super::Head;
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, thiserror::Error)]
#[error("{self:?}")]
pub enum Error {
    Timeout,
    Internal { message: String },
    PageNotFound { uri: String },
    SetNotFound,
}

impl Error {
    #[server]
    fn title(&self) -> String {
        let title = match self {
            Self::Internal { .. } => "Unexpected Error",
            Self::Timeout => "Request Timeout",
            Self::PageNotFound { .. } => "Page Not Found",
            Self::SetNotFound => "Set Not Found",
        };
        format!("{title} - QUUXLET")
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
                            Self::Internal { message } => {
                                h1 {{ "Internal Server Error!" }}
                                p {{ message }}
                            },
                            Self::Timeout => {
                                h1 {{ "Request Timeout!" }}
                            },
                            Self::PageNotFound { uri } => {
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
impl From<Error> for axum::http::StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::Internal { .. } => Self::INTERNAL_SERVER_ERROR,
            Error::Timeout => Self::REQUEST_TIMEOUT,
            Error::PageNotFound { .. } | Error::SetNotFound => Self::NOT_FOUND,
        }
    }
}
