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

    fn dom_representation(&self) -> DomRepresentation {
        DomRepresentation::Many(self.0.iter().flat_map(Item::dom_representation).collect())
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

impl<T: Item> Hydrate for Many<T> {
    fn hydrate(self)
    where
        Self: Sized,
    {
        for item in self.0 {
            item.hydrate();
        }
    }
}
