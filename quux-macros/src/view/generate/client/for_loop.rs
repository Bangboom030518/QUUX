use super::super::internal::prelude::*;
use crate::view::parse::prelude::*;
use for_loop::Iterable;

impl ForLoop {
    fn binding_code(&mut self, index: usize) -> TokenStream {
        let Html {
            components,
            for_loop_components,
            ..
        } = (*self.item.clone()).into();

        let index = syn::Index::from(index);

        self.bindings = components.bindings().into_iter().cloned().collect();

        let bindings: TokenStream = components
            .0
            .iter()
            .enumerate()
            .map(if self.is_reactive() {
                |(index, component): (_, &Component)| {
                    let index = syn::Index::from(index);
                    let Some(binding) = component.binding.clone() else {
                        return TokenStream::new()
                    };
                    quote! {
                        #binding = std::rc::Rc::new(std::cell::RefCell::new(internal.#index));
                    }
                }
            } else {
                |(index, component): (_, &Component)| {
                    let index = syn::Index::from(index);
                    let Some(binding) = component.binding.clone() else {
                        return TokenStream::new()
                    };
                    quote! {
                        #binding = internal.#index;
                    }
                }
            })
            .collect();

        let types = components.types();
        let children_type = quote! {
            (#(Vec<#types>,)*)
        };
        let indices = (0..types.len()).map(syn::Index::from);

        let vecs = std::iter::repeat(quote! {
            Vec::new(),
        })
        .take(types.len());

        quote! {{
            let mut internal: #children_type = (#(#vecs)*);
            for components in for_loop_components.#index {
                #({
                    let child = components.#indices;
                    internal.#indices.push(child.render().component);
                })*
            }
            #bindings;
        }}
    }

    fn pop_code(&self) -> TokenStream {
        let id = self.id;
        let binding_code: TokenStream = self
            .bindings
            .iter()
            .map(|binding| {
                quote! {
                    #binding.borrow_mut().pop()
                }
            })
            .collect();
        quote! {{
            quux::dom::get_reactive_for_loop_element(*id, #id, index).remove();
            #binding_code;
        }}
    }

    fn push_code(&self) -> TokenStream {
        let id = self.id;
        let binding_code: TokenStream = self
            .bindings
            .iter()
            .map(|binding| {
                quote! {
                    #binding.borrow_mut().push(component.clone())
                }
            })
            .collect();
        quote! {{
            quux::dom::get_reactive_for_loop_element(*id, #id, index).remove();
            #binding_code;
        }}
    }

    fn list_store_code(&self) -> TokenStream {
        let Iterable::Reactive(store) = self.iterable.clone() else {
            return TokenStream::new()
        };
        assert!(
            !matches!(*self.item, Item::Expression(_)),
            "reactive for loops must contain either elements or components"
        );
        let pop_code = self.pop_code();
        let push_code = self.push_code();
        let binding_code: TokenStream = self
            .bindings
            .iter()
            .map(|binding| {
                quote! {
                    let #binding = std::rc::Rc::clone(&#binding);
                }
            })
            .collect();
        quote! {
            quux::store::List::on_change(&#store, {
                let id = Rc::clone(&id);
                #binding_code;
                move |event| match event {
                    quux::store::list::Event::Push(_) => #push_code,
                    quux::store::list::Event::Pop(_, index) => #pop_code,
                }
            })
        }
    }

    pub fn reactivity(&mut self, index: usize) -> TokenStream {
        let binding_code = self.binding_code(index);
        let reactivity = self.list_store_code();
        quote! {
            #binding_code;
            #reactivity;
        }
    }
}
