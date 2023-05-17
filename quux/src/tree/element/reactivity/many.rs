#[client]
use super::Reactivity;
use crate::internal::prelude::*;

#[client]
#[derive(Clone)]
pub struct Many<'a, T, F, I>
where
    F: Mapping<'a, T, I>,
    I: Item,
{
    list: store::List<T>,
    mapping: F,
    _phantom: PhantomData<&'a I>,
}

#[client]
impl<'a, T, F, I> Debug for Many<'a, T, F, I>
where
    F: Mapping<'a, T, I>,
    I: Item,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Many").finish()
    }
}

#[client]
impl<'a, T, F, I> Many<'a, T, F, I>
where
    F: Mapping<'a, T, I>,
    I: Item,
{
    pub const fn new(list: store::List<T>, mapping: F) -> Self {
        Self {
            list,
            mapping,
            _phantom: PhantomData,
        }
    }
}

#[client]
impl<'a, T, F, I> Reactivity for Many<'a, T, F, I>
where
    F: Mapping<'a, T, I>,
    I: Item,
{
    fn apply(self: Box<Self>, element: Rc<web_sys::Element>) {
        use store::list::Event;

        let mut mapping = self.mapping;

        self.list.on_change(move |event| match event {
            Event::Pop => element
                .last_element_child()
                .expect_internal("get last element of `ReactiveMany` list")
                .remove(),
            Event::Push(index, new) => {
                let mut new_element = mapping(index, new);
                let dom_element = new_element.create_element(true);
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

pub trait Mapping<'a, T, I>:
    for<'b> FnMut(Store<usize>, &'b T) -> Element<'a, I> + 'static
where
    I: Item,
{
}

impl<'a, T, F, I> Mapping<'a, T, I> for F
where
    F: for<'b> FnMut(Store<usize>, &'b T) -> Element<'a, I> + 'static,
    I: Item,
{
}
