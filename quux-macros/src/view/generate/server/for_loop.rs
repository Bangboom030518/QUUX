use super::super::GLOBAL_ID;
use super::Html;
use crate::view::parse::element::{children::ForLoopIterable, ForLoop};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::sync::atomic::Ordering::Relaxed;

impl ToTokens for ForLoop {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            pattern,
            iterable,
            mut item,
        } = self.clone();
        // TODO: components!!!
        let reactive: bool;
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
        let id = GLOBAL_ID.fetch_add(1, Relaxed);
        if reactive {
            item.insert_for_loop_id(
                // `[scope id].for[for loop id].[for loop index]`
                crate::parse(quote! {
                    format!("{}.for{}.{}", &scope_id, #id, index)
                }),
            );
        }
        let Html(html) = (*item).into();

        tokens.extend(quote! {{
            let mut components = Vec::<quux::ClientComponentNode<Self::ComponentEnum>>::new();
            let html = (#iterable).enumerate().map(|(index, #pattern)| {
                ToString::to_string(&#html)
            }).collect::<String>();
            for_loop_children.push(components);
            html
        }});
    }
}
