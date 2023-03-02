use crate::view::parse::prelude::*;
use element::children::{ForLoop, ForLoopIterable};
use proc_macro2::TokenStream;
use quote::quote;

impl ForLoop {
    fn binding_code(&self) -> TokenStream {
        // TODO: handle reactive fors properly
        let Item::Component(Component { binding, .. }) = *self.item.clone() else {
            return TokenStream::new()
        };
        let Some(binding) = binding else {
            return TokenStream::new()
        };
        quote! {
            {
                let mut internal: Vec<_> = Vec::new();
                for child in for_loop_children.next().expect_internal("retrieve for loop children: client and server for loop lists don't match") {
                    let mut component = child.component;
                    component.render(child.render_context);
                    internal.push(component.try_into().expect_internal("retrieve for loop children: client and server for loop lists don't match"))
                }
                #binding = internal;
            }
        }
    }

    // TODO: rename
    fn reactive_for_code(&self) -> TokenStream {
        // TODO: assert not expression child
        let ForLoopIterable::Reactive(store) = self.iterable.clone() else {
            return TokenStream::new()
        };
        quote! {
            let mut currrent_component_nodes: Vec<_> = Vec::new();
            for_loop_children.push(currrent_component_nodes);

            quux::store::List::on_change(&#store, |event| {
                match event {
                    quux::store::list::Event::Push(_) => todo!("handle push"),
                    quux::store::list::Event::Pop(_) => {
                        let element = quux::dom::get_reactive_element(context.id)
                        // quux::dom::get_reactive_element(&*scope_id, #scoped_id)
                    }
                }
            })
        }
    }

    pub fn reactivity_code(&self) -> TokenStream {
        let reactivity = self.reactive_for_code();
        let binding_code = self.binding_code();
        quote! {
            #binding_code;
            #reactivity;
        }
    }
}
