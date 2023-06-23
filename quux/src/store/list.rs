use super::ArcCell;
use crate::{
    internal::prelude::*,
    tree::{self, element::reactivity},
};

pub type Callback<T> = Box<dyn FnMut(Event<T>) + 'static>;

#[derive(Serialize, Deserialize, Default)]
pub struct List<T> {
    value: ArcCell<Vec<(Store<usize>, T)>>,
    #[serde(skip)]
    listeners: ArcCell<Vec<Callback<T>>>,
}

impl<T> List<T> {
    #[must_use]
    pub fn new(values: Vec<T>) -> Self {
        Self {
            value: Arc::new(RefCell::new(
                values
                    .into_iter()
                    .enumerate()
                    .map(|(index, value)| (Store::new(index), value))
                    .collect(),
            )),
            listeners: Arc::new(RefCell::new(Vec::new())),
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
        let index_store = Store::new(self.length());
        let mut listeners = self.listeners.borrow_mut();
        for listener in listeners.iter_mut() {
            listener(Event::Push(index_store.clone(), &value));
        }
        self.value.borrow_mut().push((index_store, value));
    }

    #[must_use]
    pub fn length(&self) -> usize {
        self.value.borrow().len()
    }

    #[allow(clippy::must_use_candidate)]
    pub fn pop(&self) -> Option<T> {
        let mut listeners = self.listeners.borrow_mut();
        let value = self.value.borrow_mut().pop()?;
        for listener in listeners.iter_mut() {
            listener(Event::Pop);
        }
        Some(value.1)
    }

    #[allow(clippy::must_use_candidate)]
    pub fn swap(&self, a: usize, b: usize) {
        let mut listeners = self.listeners.borrow_mut();
        let mut list = self.value.borrow_mut();

        list[a].0.set(b);
        list[b].0.set(a);
        list.swap(a, b);

        for listener in listeners.iter_mut() {
            listener(Event::Swap(a, b));
        }
    }

    #[allow(clippy::must_use_candidate)]
    pub fn remove(&self, index: usize) -> T
    where
        T: Clone,
    {
        let mut listeners = self.listeners.borrow_mut();
        let mut list = self.value.borrow_mut();
        let value = list.remove(index);

        for (store, _) in list.get_mut(index..).unwrap_or(&mut []) {
            let previous_index = *store.get();
            store.set(previous_index - 1);
        }

        for listener in listeners.iter_mut() {
            listener(Event::Remove(index));
        }
        value.1
    }

    /// Gets the interior value
    #[must_use]
    pub fn get(&self, index: usize) -> Option<T>
    where
        T: Clone,
    {
        self.value
            .borrow()
            .get(index)
            .cloned()
            .map(|(_, value)| value)
    }

    pub fn into_many<'a, F, I>(&self, mapping: &mut F) -> item::Many<tree::Element<'a, I>>
    where
        I: Item + 'a,
        F: reactivity::many::Mapping<'a, T, I>,
        T: 'a,
    {
        self.value
            .borrow()
            .iter()
            .map(|(index, value)| mapping(index.clone(), value))
            .collect::<Many<_>>()
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            listeners: Arc::clone(&self.listeners),
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
    Push(Store<usize>, &'a T),
    Pop,
    Remove(usize),
    Swap(usize, usize),
}
