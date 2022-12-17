use serde::{Deserialize, Serialize};
use std::fmt;

pub type StoreCallback<'a, T> = dyn FnMut(&T, &T) + 'a;

#[derive(Serialize, Deserialize)]
pub struct Store<'a, T: fmt::Display>
where
    Self: 'a,
{
    value: T,
    #[serde(skip)]
    listeners: Vec<Box<StoreCallback<'a, T>>>,
}

impl<'a, T: fmt::Display> Store<'a, T> {
    /// Creates a new store.
    pub fn new(value: T) -> Self {
        Self {
            value,
            listeners: Vec::new(),
        }
    }

    /// Pushes a closure to run whenever the state is changed. The closure will recieve the previous, and new interior value.
    ///
    /// > *NOTE*: You will need to wrap any values borrowed by the closure in a [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) or similar if you plan to use it again afterwards.
    pub fn on_change<F>(&mut self, listener: F)
    where
        F: FnMut(&T, &T) + 'a,
    {
        self.listeners.push(Box::new(listener));
    }

    /// Returns a reference to the current value
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Sets interior value to `value`.
    pub fn set(&mut self, value: T) {
        for listener in &mut self.listeners {
            listener(&self.value, &value);
        }
        self.value = value;
    }
}
impl<'a, T: fmt::Display> fmt::Display for Store<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[test]
fn store_test() {
    use std::cell::RefCell;

    let result_1: RefCell<Vec<(u8, u8)>> = RefCell::new(Vec::new());
    let result_2: RefCell<Vec<(u8, u8)>> = RefCell::new(Vec::new());
    let mut store = Store::new(0);
    store.on_change(|&previous, &current| result_1.borrow_mut().push((previous, current)));

    store.on_change(|&previous, &current| {
        result_2.borrow_mut().push((previous + 10, current + 10));
    });
    for _ in 0..3 {
        store.set(store.get() + 1);
    }
    assert_eq!(result_1.borrow().as_slice(), &[(0, 1), (1, 2), (2, 3)]);
    assert_eq!(
        result_2.borrow().as_slice(),
        &[(10, 11), (11, 12), (12, 13)]
    );
}
