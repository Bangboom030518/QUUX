use crate::internal::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawHtml(String);

impl RawHtml {
    pub fn new<S>(html: S) -> Self
    where
        S: Display,
    {
        Self(html.to_string())
    }
}

impl Display for RawHtml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Item for RawHtml {
    fn insert_id(&mut self, id: u64) -> u64 {
        id
    }

    #[cfg_client]
    fn dom_representation(&mut self) -> DomRepresentation {
        DomRepresentation::One(
            web_sys::Range::new()
                .expect_internal("create `Range`")
                .create_contextual_fragment(&self.0)
                .expect_internal("create fragment for `RawHtml`")
                .into(),
        )
    }
}

#[cfg_client]
impl From<RawHtml> for DomRepresentation {
    fn from(mut value: RawHtml) -> Self {
        value.dom_representation()
    }
}
