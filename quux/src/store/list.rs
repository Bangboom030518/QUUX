use super::RcCell;
use crate::internal::prelude::*;

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
    pub fn length(&self) -> usize {
        self.value.borrow().len()
    }

    #[allow(clippy::must_use_candidate)]
    pub fn pop(&self) -> Option<T> {
        let index = self.length() - 1;
        let mut listeners = self.listeners.borrow_mut();
        let value = self.value.borrow_mut().pop()?;
        for listener in listeners.iter_mut() {
            listener(Event::Pop(&value, index));
        }
        Some(value)
    }

    /// Gets the interior value
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        self.value.borrow().get(index).cloned()
    }
}

impl<T: Clone> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.value.borrow().into_iter()
    }
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
        f.debug_struct("List")
            .field("value", &self.value.borrow())
            .field("listeners", &self.listeners.borrow().len())
            .finish()
    }
}

pub enum Event<'a, T> {
    Push(&'a T),
    Pop(&'a T, usize),
    // Insert(&'a T, usize),
    // Remove(&'a T, usize),
}
