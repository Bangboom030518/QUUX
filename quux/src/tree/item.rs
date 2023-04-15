use super::{Hydrate, ToString2};
use crate::internal::prelude::*;

pub trait Item: ToString2 + Hydrate {}

// impl<T: Component> Item for Output<T> {
//     fn hydrate(&self) {
//         self.element.hydrate()
//     }

//     fn to_string(&self) -> String {
//         ToString::to_string(&self.element)
//     }
// }

impl<T: Children> Item for Element<T> {}

impl Item for String {}
impl Hydrate for String {}
