use quux::prelude::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ServerError {
    Timeout,
    Internal,
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
            h1 {{ "Internal Server Error!" }}
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
        }
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// impl axum::response::IntoResponse for ServerError {
//     fn into_response(self) -> axum::response::Response {
//         use axum::http::StatusCode;
//         (StatusCode::from(self), self.render_to_string()).into_response()
//     }
// }
