// TODO: list stores!!!!!!!

pub use list::List;
use serde::{Deserialize, Serialize};
use std::{
    cell::{Ref, RefCell},
    fmt,
    rc::Rc,
};

pub mod list;

pub type Callback<T> = Box<dyn FnMut(&T, &T) + 'static>;
type RcCell<T> = Rc<RefCell<T>>;

#[derive(Serialize, Deserialize)]
pub struct Store<T> {
    value: RcCell<T>,
    #[serde(skip)]
    listeners: RcCell<Vec<Callback<T>>>,
}

impl<T> Store<T> {
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
        F: FnMut(&T, &T) + 'static,
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

impl<T> Clone for Store<T> {
    fn clone(&self) -> Self {
        Self {
            value: Rc::clone(&self.value),
            listeners: Rc::clone(&self.listeners),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    where
        T: fmt::Display,
    {
        write!(f, "{}", self.get())
    }
}

impl<T: fmt::Debug> fmt::Debug for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    where
        T: fmt::Debug,
    {
        write!(f, "{:?}", self.get())
    }
}
