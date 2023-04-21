use quux::prelude::*;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ServerError {
    Timeout,
    Internal,
}

impl Component for ServerError {
    fn render(self, context: Context<Self>) -> Output<Self> {
        // type Component = ServerError;
        // view! {
        //     context,
        //     h1 {{ "Internal Server Error!" }}
        // }
        h1().text("Internal Server Error!")
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
