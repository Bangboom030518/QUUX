use crate::view::parse::prelude::*;
use element::children::{ForLoop, ForLoopIterable};
use proc_macro2::TokenStream;
use quote::quote;

impl ForLoop {
    fn binding_code(&mut self) -> TokenStream {
        // TODO: handle reactive fors properly
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
                let mut internal: Vec<_> = Vec::new();
                for child in for_loop_children.next().expect_internal("retrieve for loop children: client and server for loop lists don't match") {
                    let mut component = child.component;
                    component.render(child.render_context);
                    internal.push(component.try_into().expect_internal("retrieve for loop children: client and server for loop lists don't match"))
                }
                #binding;
            }
        }
    }

    fn pop_code(&self, id: u64) -> TokenStream {
        let binding_code = self.binding.as_ref().map_or_else(TokenStream::new, |_| {
            quote! {
                binding.borrow_mut().pop()
            }
        });
        let id = id.to_string();
        quote! {{
            quux::dom::get_reactive_for_loop_element(&*scope_id, #id, index).remove();
            #binding_code;
        }}
    }

    // TODO: rename
    fn reactive_for_code(&self, id: u64) -> TokenStream {
        // TODO: assert not expression child
        let ForLoopIterable::Reactive(store) = self.iterable.clone() else {
            return TokenStream::new()
        };
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
                let scope_id = Rc::clone(&scope_id);
                #binding_code;
                move |event| match event {
                    quux::store::list::Event::Push(_) => todo!("handle push"),
                    quux::store::list::Event::Pop(_, index) => #pop_code,
                }
            })
        }
    }

    pub fn reactivity_code(&mut self, id: u64) -> TokenStream {
        let binding_code = self.binding_code();
        let reactivity = self.reactive_for_code(id);
        quote! {
            #binding_code;
            #reactivity;
        }
    }
}
