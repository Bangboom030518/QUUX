use super::RcCell;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};

pub type Callback<T> = Box<dyn FnMut(Event<T>) + 'static>;

#[derive(Serialize, Deserialize, Default)]
pub struct List<T> {
    value: RcCell<Vec<T>>,
    #[serde(skip)]
    listeners: RcCell<Vec<Callback<T>>>,
}

impl<T> List<T> {
    #[must_use]
    pub fn new(values: Vec<T>) -> Self {
        Self {
            value: Rc::new(RefCell::new(values)),
            listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Pushes a closure to run whenever the state is changed. The closure will recieve the previous, and new interior value.
    ///
    /// > *NOTE*: You will need to wrap any values borrowed by the closure in a [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) or similar if you plan to use it again afterwards.
    pub fn on_change<F>(&self, listener: F)
    where
        F: FnMut(Event<T>) + 'static,
    {
        let mut listeners = self.listeners.borrow_mut();
        listeners.push(Box::new(listener));
    }

    pub fn push(&self, value: T) {
        let mut listeners = self.listeners.borrow_mut();
        for listener in listeners.iter_mut() {
            listener(Event::Push(&value));
        }
        self.value.borrow_mut().push(value);
    }

    #[must_use]
    pub fn pop(&self) -> Option<T> {
        let mut listeners = self.listeners.borrow_mut();
        let value = self.value.borrow_mut().pop()?;
        for listener in listeners.iter_mut() {
            listener(Event::Pop(&value));
        }
        Some(value)
    }

    /// Gets the interior value
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        // TODO: clone????
        self.value.borrow().get(index).cloned()
    }

    // pub fn get_values(&self) -> Vec<T>> {
    //     self.value.borrow()
    // }
}

impl<'a, T> From<&'a List<T>> for std::cell::Ref<'a, Vec<T>> {
    fn from(value: &'a List<T>) -> Self {
        value.value.borrow()
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            listeners: Rc::clone(&self.listeners),
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // TODO: show listeners
        f.debug_list().entries(self.value.borrow().iter()).finish()
    }
}

pub enum Event<'a, T> {
    Push(&'a T),
    Pop(&'a T),
    // Insert(&'a T, usize),
    // Remove(&'a T, usize),
}
