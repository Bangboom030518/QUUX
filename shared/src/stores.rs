use serde::{Deserialize, Serialize};
use std::{
    cell::{Ref, RefCell},
    fmt,
    rc::Rc,
};

pub type StoreCallback<'a, T> = Box<dyn FnMut(&T, &T) + 'a>;
type RcCell<T> = Rc<RefCell<T>>;

#[derive(Serialize, Deserialize)]
pub struct Store<'a, T: fmt::Display>
where
    Self: 'a,
{
    value: RcCell<T>,
    #[serde(skip)]
    listeners: RcCell<Vec<StoreCallback<'a, T>>>,
}

impl<'a, T: fmt::Display> Store<'a, T> {
    /// Creates a new store.
    pub fn new(value: T) -> Self {
        Self {
            value: Rc::new(RefCell::new(value)),
            listeners: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Pushes a closure to run whenever the state is changed. The closure will recieve the previous, and new interior value.
    ///
    /// > *NOTE*: You will need to wrap any values borrowed by the closure in a [`RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html) or similar if you plan to use it again afterwards.
    pub fn on_change<F>(&self, listener: F)
    where
        F: FnMut(&T, &T) + 'a,
    {
        let mut listeners = self.listeners.borrow_mut();
        listeners.push(Box::new(listener));
    }

    /// Sets interior value to `value`.
    pub fn set(&self, value: T) {
        let mut listeners = self.listeners.borrow_mut();
        {
            let previous = self.value.borrow();
            for listener in listeners.iter_mut() {
                listener(&*previous, &value);
            }
        }
        *self.value.borrow_mut() = value;
    }

    /// Gets the interior value
    #[must_use]
    pub fn get(&self) -> Ref<T> {
        self.value.borrow()
    }
}

impl<'a, T: fmt::Display> fmt::Display for Store<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

impl<'a, T: fmt::Display> Clone for Store<'a, T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            listeners: Rc::clone(&self.listeners),
        }
    }
}

#[test]
fn store_test() {
    use std::cell::RefCell;

    let result_1: RefCell<Vec<(u8, u8)>> = RefCell::new(Vec::new());
    let result_2: RefCell<Vec<(u8, u8)>> = RefCell::new(Vec::new());
    let store = Store::new(0);
    store.on_change(|&previous, &current| result_1.borrow_mut().push((previous, current)));

    store.on_change(|&previous, &current| {
        result_2.borrow_mut().push((previous + 10, current + 10));
    });
    for _ in 0..3 {
        store.set(*store.get() + 1);
    }
    assert_eq!(result_1.borrow().as_slice(), &[(0, 1), (1, 2), (2, 3)]);
    assert_eq!(
        result_2.borrow().as_slice(),
        &[(10, 11), (11, 12), (12, 13)]
    );
}
