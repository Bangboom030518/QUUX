use crate::internal::prelude::*;

pub struct Many<T: Item>(Vec<T>);

impl<T: Item> FromIterator<T> for Many<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<T: Item> Item for Many<T> {}

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
