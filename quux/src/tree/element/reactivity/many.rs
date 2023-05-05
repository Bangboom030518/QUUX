#[client]
use super::Reactivity;
use crate::internal::prelude::*;

#[client]
#[derive(Clone)]
pub struct Many<'a, T, F, I>
where
    T: Clone,
    F: FnMut(Store<usize>, T) -> Element<'a, I> + Clone,
    I: Item,
{
    list: store::List<T>,
    mapping: F,
}

#[client]
impl<'a, T, F, I> Debug for Many<'a, T, F, I>
where
    T: Clone,
    F: FnMut(Store<usize>, T) -> Element<'a, I> + Clone,
    I: Item,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Many").finish()
    }
}

#[client]
impl<'a, T, F, I> Many<'a, T, F, I>
where
    T: Clone,
    F: FnMut(Store<usize>, T) -> Element<'a, I> + Clone,
    I: Item,
{
    pub const fn new(list: store::List<T>, mapping: F) -> Self {
        Self { list, mapping }
    }
}

#[client]
impl<'a, T, F, I> Reactivity for Many<'a, T, F, I>
where
    T: Clone + 'a,
    F: FnMut(Store<usize>, T) -> Element<'a, I> + 'static + Clone,
    I: Item + 'a,
{
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>) {
        use store::list::Event;

        let mut mapping = self.mapping.clone();
        self.list.on_change(move |event| match event {
            Event::Pop => element
                .last_element_child()
                .expect_internal("get last element of `ReactiveMany` list")
                .remove(),
            Event::Push(index, new) => {
                let new_element = mapping(index, new.clone());
                let dom_element = new_element.create_element();
                element
                    .append_child(&dom_element)
                    .expect_internal("append child");
            }
            Event::Remove(index) => {
                element
                    .children()
                    .item(
                        #[allow(clippy::cast_possible_truncation)]
                        {
                            index as u32
                        },
                    )
                    .expect_internal("get element of `ReactiveMany` list")
                    .remove();
            }
        });
    }
}
