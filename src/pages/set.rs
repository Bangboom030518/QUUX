use super::server_error::ServerError;
use crate::components::Flashcards;
use quux::prelude::*;
// note: required for `quux::render::Output<set::Set>` to implement `Into<EnumRenderOutput<set::ComponentEnum>>`

// init_components!($ Set);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ComponentEnum {
    Component0(Set),
    Component1(quux::initialisation_script::InitialisationScript<ComponentEnum>),
}
impl quux::component::Enum for ComponentEnum {
    fn render(self, context: render::Context<Self>) -> quux::component::EnumRenderOutput<Self> {
        match self {
            Self::Component0(component) => component.render(context.into()).into(),
            Self::Component1(component) => component.render(context).into(),
        }
    }
}
impl From<Set> for ComponentEnum {
    fn from(value: Set) -> Self {
        Self::Component0(value)
    }
}
impl TryFrom<ComponentEnum> for Set {
    type Error = ();
    fn try_from(value: ComponentEnum) -> Result<Self, Self::Error> {
        if let ComponentEnum::Component0(component) = value {
            Ok(component)
        } else {
            Err(())
        }
    }
}
impl From<quux::initialisation_script::InitialisationScript<ComponentEnum>> for ComponentEnum {
    fn from(value: quux::initialisation_script::InitialisationScript<ComponentEnum>) -> Self {
        Self::Component1(value)
    }
}
impl TryFrom<ComponentEnum> for quux::initialisation_script::InitialisationScript<ComponentEnum> {
    type Error = ();
    fn try_from(value: ComponentEnum) -> Result<Self, Self::Error> {
        if let ComponentEnum::Component1(component) = value {
            Ok(component)
        } else {
            Err(())
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Set(super::super::Set);

#[cfg(not(target_arch = "wasm32"))]
impl Set {
    pub async fn new(pool: &sqlx::Pool<sqlx::Sqlite>, set_id: &str) -> Result<Self, ServerError> {
        match super::super::Set::fetch(pool, set_id).await {
            Ok(set) => Ok(Self::init(set)),
            Err(error) => Err(match error {
                sqlx::Error::RowNotFound => todo!(), // (StatusCode::NOT_FOUND, "Set not found :(".to_string()),
                _ => ServerError::init(Box::new(error)),
            }),
        }
    }
}

impl Component for Set {
    type Props = super::super::Set;
    type ComponentEnum = crate::ComponentEnum;

    fn init(set: super::super::Set) -> Self {
        Self(set)
    }

    fn render(self, context: render::Context<Self::ComponentEnum>) -> render::Output<Self> {
        view! {
            context,
            html(lang="en") {
                head {
                    meta(charset="UTF-8") {}
                    meta("http-equiv"="X-UA-Compatible", content="IE=edge") {}
                    meta(name="viewport", content="width=device-width, initial-scale=1.0") {}
                    style {
                        { include_str!("../../dist/output.css") }
                    }
                    title {{ "Document" }}
                }
                body {
                    h1 {{ "Welcome to Quuxlet" }}
                    @Flashcards(self.0.terms.clone())
                    @InitialisationScript<Self::ComponentEnum>(include_str!("../../dist/init.js"))
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl axum::response::IntoResponse for Set {
    fn into_response(self) -> axum::response::Response {
        axum::response::Html::from(self.render_to_string()).into_response()
    }
}
