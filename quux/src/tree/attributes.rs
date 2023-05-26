use super::DisplayStore;
use crate::internal::prelude::*;

#[derive(Default, Clone)]
pub struct Attributes {
    pub attributes: HashMap<String, String>,
    pub reactive_attributes: HashMap<String, DisplayStore>,
}

impl Debug for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attributes")
            .field("attributes", &self.attributes)
            .field(
                "reactive_attributes",
                &self
                    .attributes
                    .iter()
                    .map(|(key, value)| (key, value.to_string()))
                    .collect::<HashMap<_, _>>(),
            )
            .finish()
    }
}

impl Attributes {
    #[must_use]
    pub fn new(
        attributes: HashMap<String, String>,
        reactive_attributes: HashMap<String, DisplayStore>,
    ) -> Self {
        Self {
            attributes,
            reactive_attributes,
        }
    }
}

impl Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (key, value) in &self.attributes {
            write!(f, "{key}=\"{}\"", value.replace('"', r#"\""#))?;
        }
        Ok(())
    }
}
