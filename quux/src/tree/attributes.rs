use crate::internal::prelude::*;
use super::DisplayStore;

#[derive(Default, Clone)]
pub struct Attributes {
    pub attributes: HashMap<String, String>,
    pub reactive_attributes: HashMap<String, DisplayStore>,
}

impl Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in &self.attributes {
            write!(f, "{key}=\"{}\"", value.replace('"', r#"\""#))?;
        }
        Ok(())
    }
}
