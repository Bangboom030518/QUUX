use crate::internal::prelude::*;

#[derive(Clone, Debug)]
pub struct Pair<A: Item, B: Item>(pub A, pub B);

impl<A: Item, B: Item> Hydrate for Pair<A, B> {
    fn hydrate(self) {
        self.0.hydrate();
        self.1.hydrate();
    }
}

impl<A: Item, B: Item> Display for Pair<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)?;
        Display::fmt(&self.1, f)?;
        Ok(())
    }
}

impl<A: Item, B: Item> Item for Pair<A, B> {
    fn insert_id(&mut self, id: u64) -> u64 {
        let id = self.0.insert_id(id);
        self.1.insert_id(id + 1)
    }

    fn dom_representation(&self) -> DomRepresentation {
        DomRepresentation::Many(
            self.0
                .dom_representation()
                .into_iter()
                .chain(self.1.dom_representation().into_iter())
                .collect(),
        )
    }
}
