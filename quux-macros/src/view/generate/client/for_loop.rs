use super::super::internal::prelude::*;
use crate::view::parse::prelude::*;

impl ForLoop {
    fn binding_code(&mut self) -> TokenStream {
        let Item::Component(Component { binding, .. }) = *self.item.clone() else {
            return TokenStream::new()
        };
        let Some(binding) = binding else {
            return TokenStream::new()
        };
        self.binding = Some(binding.clone());
        let binding = if self.is_reactive() {
            quote! {
                #binding = std::rc::Rc::new(std::cell::RefCell::new(internal));
            }
        } else {
            quote! {
                #binding = internal;
            }
        };
        quote! {
            {
                todo!()
                // let mut internal: Vec<_> = Vec::new();
                // for child in for_loop_children.#index {
                //     quux::component::Component::render(child.component.clone(), child.render_context);
                //     internal.push(child.component)
                // }
                // #binding;
            }
        }
    }

    fn pop_code(&self, id: u64) -> TokenStream {
        let binding_code = self.binding.as_ref().map_or_else(TokenStream::new, |_| {
            quote! {
                binding.borrow_mut().pop()
            }
        });
        quote! {{
            quux::dom::get_reactive_for_loop_element(*id, #id, index).remove();
            #binding_code;
        }}
    }

    fn list_store_code(&self, id: u64) -> TokenStream {
        let ForLoopIterable::Reactive(store) = self.iterable.clone() else {
            return TokenStream::new()
        };
        assert!(
            !matches!(*self.item, Item::Expression(_)),
            "reactive for loops must contain either elements or components"
        );
        let pop_code = self.pop_code(id);
        let binding_code = self
            .binding
            .as_ref()
            .map_or_else(TokenStream::new, |binding| {
                quote! {
                    let binding = std::rc::Rc::clone(&#binding);
                }
            });
        quote! {
            quux::store::List::on_change(&#store, {
                let id = Rc::clone(&id);
                #binding_code;
                move |event| match event {
                    quux::store::list::Event::Push(_) => todo!("handle push"),
                    quux::store::list::Event::Pop(_, index) => #pop_code,
                }
            })
        }
    }

    pub fn reactivity(&mut self, id: u64) -> TokenStream {
        let binding_code = self.binding_code();
        let reactivity = self.list_store_code(id);
        quote! {
            #binding_code;
            #reactivity;
        }
    }
}
