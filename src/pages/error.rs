use super::Head;
use quux::prelude::*;

#[derive(Debug)]
pub enum Database {
    NotFound,
    Internal(Box<dyn std::error::Error + Send + Sync>),
}

#[server]
impl From<sqlx::Error> for Database {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            error => Self::Internal(Box::new(error)),
        }
    }
}

#[server]
impl warp::reject::Reject for Database {}

#[derive(Debug)]
pub struct NotFound(pub http::Uri);

#[server]
impl warp::reject::Reject for NotFound {}

#[derive(Clone, Debug, Serialize, Deserialize, thiserror::Error)]
#[error("{self:?}")]
pub enum Error {
    Timeout,
    Internal { message: String },
    PageNotFound { uri: String },
    SetNotFound,
}

#[server]
impl Error {
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
                                h1 {{ "Set not found!" }}
                            }
                        }
                    }
                    @InitialisationScript(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}

impl From<Error> for http::StatusCode {
    fn from(value: Error) -> Self {
        match value {
            Error::Internal { .. } => Self::INTERNAL_SERVER_ERROR,
            Error::Timeout => Self::REQUEST_TIMEOUT,
            Error::PageNotFound { .. } | Error::SetNotFound => Self::NOT_FOUND,
        }
    }
}

#[server]
impl From<warp::Rejection> for Error {
    fn from(value: warp::Rejection) -> Self {
        // TODO: internal errors + not found?
        if let Some(error) = value.find::<Database>() {
            return match error {
                Database::Internal(error) => Self::Internal {
                    message: error.to_string(),
                },
                Database::NotFound => Self::SetNotFound,
            };
        }

        if let Some(NotFound(uri)) = value.find::<NotFound>() {
            return Self::PageNotFound {
                uri: uri.to_string(),
            };
        }

        Self::Internal {
            message: format!("{value:?}"),
        }
    }
}

#[server]
impl warp::reject::Reject for Error {}
