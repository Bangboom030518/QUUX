use crate::view::parse::element::{children::ForLoopIterable, ForLoop};
use proc_macro2::TokenStream;
use quote::quote;

impl From<ForLoop> for TokenStream {
    fn from(
        ForLoop {
            pattern,
            iterable,
            item,
        }: ForLoop,
    ) -> Self {
        // TODO: components!!!
        let reactive: bool;
        let super::super::Data {
            component_nodes,
            html,
            component_constructors,
        } = (*item).into();
        let iterable = match iterable {
            ForLoopIterable::Static(iterable) => {
                reactive = false;
                quote! {
                    #iterable
                }
            }
            ForLoopIterable::Reactive(iterable) => {
                reactive = true;
                quote! {
                    (std::cell::Ref::<Vec<_>>::from(&#iterable)).iter().cloned()
                }
            }
        };
        let id_addition_code = if reactive {
            quote! {
                todo!()
            }
        } else {
            TokenStream::new()
        };
        // id = for#(for-id).(index)
        quote! {{
            let mut currrent_component_nodes: Vec<_> = Vec::new();
            let html = (#iterable).map(|#pattern| {
                #(#component_constructors);*;
                #(currrent_component_nodes.push(#component_nodes.clone()));*;
                #id_addition_code
                String::from(#html)
            }).collect::<String>();
            for_loop_children.push(currrent_component_nodes);
            html
        }}
    }
}
