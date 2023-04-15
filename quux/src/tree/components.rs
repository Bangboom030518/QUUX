use crate::internal::prelude::*;

struct CombinedTuples<A, B> where A: Components, B: Components {
    a: A,
    b: B,
}

pub trait Components {
    fn hydrate(&self);

    fn combine<T>(self, other: T) -> CombinedTuples<Self, T>
    where
        Self: Sized,
        T: Components,
    {
        CombinedTuples {
            a: self,
            b: other
        }
    }
}

impl Components for () {
    fn hydrate(&self) {}
}

impl<A> Components for (SerializedComponent<A>,)
where
    A: Component + Clone,
{
    fn hydrate(&self) {
        let Output { element, .. } = self.0.clone().render();
        element.hydrate();
    }
}

impl<A, B> Components for (SerializedComponent<A>, SerializedComponent<B>)
where
    A: Component + Clone,
    B: Component + Clone,
{
    fn hydrate(&self) {
        let Output { element, .. } = self.0.clone().render();
        element.hydrate();
        let Output { element, .. } = self.1.clone().render();
        element.hydrate();
    }
}

impl<T> Components for Option<T>
where
    T: Components,
{
    fn hydrate(&self) {
        match self {
            Some(components) => components.hydrate(),
            None => ()
        }
    }
}
