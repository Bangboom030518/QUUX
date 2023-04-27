// use super::Hydrate;
// use crate::internal::prelude::*;

// pub struct ComponentNode<T: Component>(pub T);

// impl<T: Component + Clone> Display for ComponentNode<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         self.0.clone().render(Context::new()).fmt(f)
//     }
// }

// impl<T: Component + Clone> Hydrate for ComponentNode<T> {
//     #[client]
//     fn hydrate(self) {
//         self.0.render(Context::new()).hydrate();
//     }
// }

// impl<T: Component + Clone> Item for ComponentNode<T> {
//     fn insert_id(&mut self, id: u64) {
        
//     }
// }
