use super::DisplayStore;
use crate::internal::prelude::*;

pub trait Children {
    const SELF_CLOSING: bool = false;

    fn to_string(&self) -> String;

    fn hydrate(&self);
}

impl Children for DisplayStore {
    fn to_string(&self) -> String {
        ToString::to_string(self)
    }

    fn hydrate(&self) {
        todo!()
    }
}

pub struct SelfClosing;

impl Children for SelfClosing {
    const SELF_CLOSING: bool = true;

    fn to_string(&self) -> String {
        String::new()
    }

    fn hydrate(&self) {}
}

impl Children for () {
    fn to_string(&self) -> String {
        String::new()
    }

    fn hydrate(&self) {}
}

impl<A: Item> Children for (A,) {
    fn to_string(&self) -> String {
        self.0.to_string()
    }

    fn hydrate(&self) {
        self.0.hydrate();
    }
}

impl<A: Item, B: Item> Children for (A, B) {
    fn to_string(&self) -> String {
        self.0.to_string() + &self.1.to_string()
    }

    fn hydrate(&self) {
        self.0.hydrate();
        self.1.hydrate();
    }
}
