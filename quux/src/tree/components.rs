use crate::internal::prelude::*;
use super::BoxedComponents;

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

impl From<Vec<BoxedComponents>> for BoxedComponents {
    fn from(value: Vec<BoxedComponents>) -> Self {
        value.iter().fold(Vec::with_capacity(value.len() * 2),
              |mut previous, current| { acc.extend(&[p.0, p.1]); acc })    
    }
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
