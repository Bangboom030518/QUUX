// use crate::internal::prelude::*;
use std::iter::{once, Once};

pub enum DomRepresentation {
    One(web_sys::Node),
    Many(Vec<web_sys::Node>),
    None,
}

pub enum IntoIter {
    Once(Once<web_sys::Node>),
    Many(std::vec::IntoIter<web_sys::Node>),
    None,
}

impl Iterator for IntoIter {
    type Item = web_sys::Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Once(once) => once.next(),
            Self::Many(many) => many.next(),
            Self::None => None,
        }
    }
}

impl IntoIterator for DomRepresentation {
    type Item = web_sys::Node;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Self::One(one) => IntoIter::Once(once(one)),
            Self::Many(many) => IntoIter::Many(many.into_iter()),
            Self::None => IntoIter::None,
        }
    }
}
