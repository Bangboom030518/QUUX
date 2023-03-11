use super::Html;
use crate::view::parse::element::{children::ForLoopIterable, ForLoop};
use proc_macro2::TokenStream;
use quote::quote;

impl ForLoop {
    pub fn tokens(&self, id: u64) -> TokenStream {
        let Self {
            pattern,
            iterable,
            mut item,
            ..
        } = self.clone();
        let iterable = match iterable {
            ForLoopIterable::Static(iterable) => quote! { #iterable },
            ForLoopIterable::Reactive(iterable) => {
                let id = id.to_string();
                item.insert_for_loop_id(
                    crate::parse(quote! {
                        format!("{}.{}.{}", &root_id, #id, index)
                    }),
                );
                quote! {
                    (std::cell::Ref::<Vec<_>>::from(&#iterable)).iter().cloned()
                }
            }
        };
        let Html(html) = (*item).into();

        quote! {{
            let mut components = Vec::<quux::ClientComponentNode<Self::ComponentEnum>>::new();
            let html = (#iterable).enumerate().map(|(index, #pattern)| {
                ToString::to_string(&#html)
            }).collect::<String>();
            for_loop_children.push(components);
            html
        }}
    }
}
