use crate::internal::prelude::*;

#[derive(Clone, Debug)]
pub struct Many<T: Item>(Vec<T>);

impl<T: Item> FromIterator<T> for Many<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T: Item> Item for Many<T> {
    fn insert_id(&mut self, mut id: u64) -> u64 {
        for item in &mut self.0 {
            id = item.insert_id(id + 1);
        }
        id
    }

    #[cfg_client]
    fn hydrate(&mut self) {
        for item in &mut self.0 {
            item.hydrate();
        }
    }

    #[cfg_client]
    fn dom_representation(&mut self) -> DomRepresentation {
        DomRepresentation::Many(
            self.0
                .iter_mut()
                .flat_map(Item::dom_representation)
                .collect(),
        )
    }
}

impl<T: Item> Display for Many<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.0 {
            write!(f, "{item}")?;
        }
        Ok(())
    }
}
