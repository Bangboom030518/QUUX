use super::Hydrate;
use crate::internal::prelude::*;

pub trait Children: Display + Hydrate {
    const SELF_CLOSING: bool = false;
}

impl<T: Display> Hydrate for Store<T> {
    fn hydrate(&self) {
        todo!()
    }
}

impl<T: Item> Children for T {}

pub struct SelfClosing;

impl Display for SelfClosing {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Hydrate for SelfClosing {}

impl Children for SelfClosing {
    const SELF_CLOSING: bool = true;
}

pub struct Empty;

impl Display for Empty {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl Hydrate for Empty {}

impl Children for Empty {}

pub struct Pair<A: Children, B: Children>(pub A, pub B);

impl<A: Children, B: Children> Hydrate for Pair<A, B> {
    fn hydrate(&self) {
        self.0.hydrate();
        self.1.hydrate();
    }
}

impl<A: Children, B: Children> Display for Pair<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)?;
        self.1.fmt(f)?;
        Ok(())
    }
}

impl<A: Children, B: Children> Children for Pair<A, B> {}
