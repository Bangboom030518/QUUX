use crate::internal::prelude::*;
use super::DisplayStore;

#[derive(Default, Clone)]
pub struct Attributes {
    pub attributes: HashMap<String, String>,
    pub reactive_attributes: HashMap<String, DisplayStore>,
}

impl Attributes {
    #[must_use]
    pub fn new(attributes: HashMap<String, String>, reactive_attributes: HashMap<String, DisplayStore>) -> Self {
        Self { attributes, reactive_attributes }
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
