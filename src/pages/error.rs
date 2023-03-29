use super::Head;
use http::Uri;
use quux::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ServerError {
    Timeout,
    Internal,
    PageNotFound(String),
    SetNotFound,
}

impl ServerError {
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

impl std::error::Error for ServerError {}
impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Component for ServerError {
    fn render(self, context: Context<Self>) -> Output<Self> {
        type Component = ServerError;
        view! {
            context,
            html(lang="en") {
                @Head(self.title())
                body {
                    h1 {{ "Internal Server Error!" }}
                    @InitialisationScript(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}

#[server]
impl component::Init for ServerError {
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
impl From<ServerError> for axum::http::StatusCode {
    fn from(value: ServerError) -> Self {
        match value {
            ServerError::Internal => Self::INTERNAL_SERVER_ERROR,
            ServerError::Timeout => Self::REQUEST_TIMEOUT,
            ServerError::PageNotFound(_) | ServerError::SetNotFound => Self::NOT_FOUND,
        }
    }
}
