use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Mutex};

pub fn escape(input: &str) -> String {
    input
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

pub type Context = HashMap<String, String>;

pub struct RenderData {
    pub html: String,
}

pub trait Component {
    type Props;

    fn init(props: Self::Props) -> Self;

    #[cfg(target = "wasm")]
    fn render(&self, context: Context);

    #[cfg(not(target = "wasm"))]
    fn render(&self) -> RenderData;
}

pub struct Store<'a, T: std::fmt::Display>
where
    Self: 'a,
{
    value: T,
    listeners: Vec<Box<dyn FnMut(&T, &T) + 'a>>,
}

impl<'a, T: std::fmt::Display> Store<'a, T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            listeners: Vec::new(),
        }
    }

    fn on_change<F>(&mut self, listener: F)
    where
        F: FnMut(&T, &T) + 'a,
    {
        self.listeners.push(Box::new(listener));
    }

    fn get(&self) -> &T {
        &self.value
    }

    fn set(&mut self, value: T) {
        for listener in self.listeners.iter_mut() {
            listener(&self.value, &value);
        }
        self.value = value;
    }
}

impl<'a, T: std::fmt::Display> std::fmt::Display for Store<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub fn init_app<T, P>(component: T)
where
    T: Component<Props = P>,
{
}

#[test]
fn store_test() {
    let result_1: Mutex<Vec<(u8, u8)>> = Mutex::new(Vec::new());
    let result_2: Mutex<Vec<(u8, u8)>> = Mutex::new(Vec::new());
    // let mut a = |&previous, &current| result_1.push((previous, current));
    // let mut b = |&previous, &current| result_2.push((previous + 10, current + 10));
    {
        let mut store = Store::new(0);
        store.on_change(move |&previous, &current| {
            (result_1).lock().unwrap().push((previous, current))
        });

        store.on_change(|&previous, &current| {
            (result_2)
                .lock()
                .unwrap()
                .push((previous + 10, current + 10))
        });

        for _ in 0..3 {
            store.set(store.get() + 1);
        }
    }

    assert_eq!(
        result_1.lock().unwrap().as_slice(),
        &[(0, 1), (1, 2), (2, 3)]
    );
    assert_eq!(
        result_2.lock().unwrap().as_slice(),
        &[(10, 11), (11, 12), (12, 13)]
    );
}
