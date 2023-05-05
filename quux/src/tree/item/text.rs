use crate::internal::prelude::*;

impl Item for String {
    fn insert_id(&mut self, id: u64) -> u64 {
        id
    }

    #[client]
    fn dom_representation(&self) -> DomRepresentation {
        DomRepresentation::One(
            web_sys::Text::new_with_data(self)
                .expect_internal("failed to create text node")
                .into(),
        )
    }
}

// TODO: only escapes on the client

#[client]
impl From<String> for DomRepresentation {
    fn from(value: String) -> Self {
        Self::One(
            web_sys::Text::new_with_data(&value)
                .expect_internal("failed to create text node")
                .into(),
        )
    }
}

impl Hydrate for String {}
