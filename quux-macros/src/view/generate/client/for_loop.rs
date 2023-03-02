use crate::view::parse::prelude::*;
use element::children::ForLoop;
use proc_macro2::TokenStream;
use quote::quote;

impl ForLoop {
    pub fn get_binding_code(&self) -> TokenStream {
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
}
