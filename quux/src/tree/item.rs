use super::Hydrate;
use crate::internal::prelude::*;

pub trait Item: Display + Hydrate {}

impl<T: Children> Item for Element<T> {}

impl Item for String {}
impl Hydrate for String {}
