use super::Hydrate;
use crate::internal::prelude::*;

pub struct ComponentNode<T: Component>(T);

impl<T: Component + Clone> Display for ComponentNode<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.clone().render(todo!()).fmt(f)
    }
}

impl<T: Component + Clone> Hydrate for ComponentNode<T> {
    fn hydrate(&self) {
        self.0.clone().render(todo!()).hydrate()
    }
}
