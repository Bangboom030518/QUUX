use crate::internal::prelude::*;
pub use list::List;

pub mod list;

pub type Callback<T> = Box<dyn FnMut(&T, &T) + 'static>;
type ArcCell<T> = Arc<RefCell<T>>;

#[derive(Serialize, Deserialize)]
pub struct Store<T> {
    value: ArcCell<T>,
    #[serde(skip)]
    listeners: ArcCell<Vec<Callback<T>>>,
}

// TODO: derived stores with map

impl<T> Store<T> {
    /// Creates a new store.
    pub fn new(value: T) -> Self {
        Self {
            value: Arc::new(RefCell::new(value)),
            listeners: Arc::new(RefCell::new(Vec::new())),
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
            value: Arc::clone(&self.value),
            listeners: Arc::clone(&self.listeners),
        }
    }
}

impl<T: Display> Display for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    where
        T: Display,
    {
        write!(f, "{}", self.get())
    }
}

impl<T: Debug> Debug for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Store").field(&self.get()).finish()
    }
}
