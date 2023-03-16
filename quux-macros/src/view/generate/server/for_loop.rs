use super::super::internal::prelude::*;
use crate::view::parse::prelude::*;

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
                item.insert_for_loop_id(id);
                quote! {
                    (std::cell::Ref::<Vec<_>>::from(&#iterable)).iter().cloned()
                }
            }
        };
        let Html(html) = (*item).into();

        quote! {{
            let mut components = Vec::<quux::render::ClientComponentNode<ComponentEnum>>::new();
            let html = (#iterable).enumerate().map(|(index, #pattern)| {
                ToString::to_string(&#html)
            }).collect::<String>();
            for_loop_children.push(components);
            html
        }}
    }
}
