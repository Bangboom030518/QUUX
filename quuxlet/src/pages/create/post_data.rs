use crate::data::Term;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PostData {
    pub terms: Vec<Term>,
    pub name: String,
}

impl PostData {
    fn from_formdata(value: Vec<(String, String)>) -> Self {
        let mut data: HashMap<String, Vec<String>> = HashMap::new();
        for (key, value) in value {
            data.entry(key).or_default().push(value);
        }

        let terms = std::iter::zip(
            data.remove("term[]").unwrap_or_default(),
            data.remove("definition[]").unwrap_or_default(),
        )
        .map(|(term, definition)| Term::new(&term, &definition))
        .collect();

        let name = data
            .remove("name")
            .and_then(|mut value| value.pop())
            .and_then(|value| (!value.is_empty()).then_some(value))
            .unwrap_or_else(|| "Untitled Set".to_string());

        Self { terms, name }
    }
}

impl<'de> Deserialize<'de> for PostData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data: Vec<(String, String)> = Deserialize::deserialize(deserializer)?;

        Ok(Self::from_formdata(data))
    }
}
