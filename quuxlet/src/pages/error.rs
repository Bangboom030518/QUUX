use super::{nav_bar, Head};
use quux::prelude::*;

#[derive(Debug)]
pub enum Database {
    NotFound,
    Internal(Box<dyn std::error::Error + Send + Sync>),
}

#[cfg_server]
impl From<sqlx::Error> for Database {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => Self::NotFound,
            error => Self::Internal(Box::new(error)),
        }
    }
}

// #[cfg_server]
// impl warp::reject::Reject for Database {}

#[derive(Debug)]
pub struct NotFound(pub http::Uri);

// #[cfg_server]
// impl warp::reject::Reject for NotFound {}

#[derive(Clone, Debug, Serialize, Deserialize, thiserror::Error)]
#[error("{self:?}")]
pub enum Error {
    Timeout,
    Internal { message: String },
    PageNotFound { uri: String },
    SetNotFound,
}

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
    fn render(self, _: Context<Self>) -> impl Item {
        html()
            .attribute("lang", "en")
            .component(Head::new(&self.title()))
            .child(
                body()
                    .class("base-layout")
                    .child(nav_bar())
                    .child(main().child(match self {
                        Self::Internal { message } => Branch4::A(children((
                            h1().text("Internal Server Error!"),
                            p().text(message),
                        ))),
                        Self::Timeout => Branch4::B(h1().text("Request Timeout!")),
                        Self::PageNotFound { uri } => {
                            Branch4::C(h1().text(format!("Page '{uri}' not found!")))
                        }
                        Self::SetNotFound => Branch4::D(h1().text("Set not found!")),
                    })),
            )
            .component(InitialisationScript::init(include_str!(
                "../../dist/init.js"
            )))
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

// #[cfg_server]
// impl From<warp::Rejection> for Error {
//     fn from(value: warp::Rejection) -> Self {
//         // TODO: internal errors + not found?
//         if let Some(error) = value.find::<Database>() {
//             return match error {
//                 Database::Internal(error) => Self::Internal {
//                     message: error.to_string(),
//                 },
//                 Database::NotFound => Self::SetNotFound,
//             };
//         }

//         if let Some(NotFound(uri)) = value.find::<NotFound>() {
//             return Self::PageNotFound {
//                 uri: uri.to_string(),
//             };
//         }

//         Self::Internal {
//             message: format!("{value:?}"),
//         }
//     }
// }

// #[cfg_server]
// impl warp::reject::Reject for Error {}
