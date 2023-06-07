use crate::internal::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Text(String);

impl Text {
    pub fn new<S>(text: S) -> Self
    where
        S: Display,
    {
        Self(text.to_string())
    }
}

impl Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: sanitize html
        Display::fmt(&self.0, f)
    }
}

impl Item for Text {
    fn insert_id(&mut self, id: u64) -> u64 {
        id
    }

    #[cfg_client]
    fn dom_representation(&mut self) -> DomRepresentation {
        DomRepresentation::One(
            web_sys::Text::new_with_data(&self.0)
                .expect_internal("failed to create text node")
                .into(),
        )
    }
}

#[cfg_client]
impl From<Text> for DomRepresentation {
    fn from(mut value: Text) -> Self {
        value.dom_representation()
    }
}
