use crate::internal::prelude::*;

pub struct Pair<A: Item, B: Item>(pub A, pub B);

impl<A: Item, B: Item> Hydrate for Pair<A, B> {
    fn hydrate(self) {
        self.0.hydrate();
        self.1.hydrate();
    }
}

impl<A: Item, B: Item> Display for Pair<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)?;
        self.1.fmt(f)?;
        Ok(())
    }
}

impl<A: Item, B: Item> Item for Pair<A, B> {
    fn insert_id(&mut self, id: u64) -> u64 {
        let id = self.0.insert_id(id);
        self.1.insert_id(id + 1)
    }
}
