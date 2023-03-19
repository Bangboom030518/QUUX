use proc_macro2::Ident;

use super::super::internal::prelude::*;
use crate::view::parse::prelude::*;

impl ForLoop {
    pub fn html(self, id: u64) -> Html {
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
        let Html {
            html,
            components,
            mut for_loop_components,
        } = (*item).into();
        for_loop_components.push(components);
        let ((component_types, component_idents), component_declarations): (
            (Vec<_>, Vec<_>),
            Vec<_>,
        ) = components
            .iter()
            .map(|Component { name, ident, .. }| {
                (
                    (name, ident),
                    quote! {
                        let #ident: quux::view::SerializedComponent<#name>;
                    },
                )
            })
            .unzip();
        let ident: Ident = todo!();
        Html {
            html: parse_quote! {{
                let (html, components): (String, Vec<_>) = (#iterable).enumerate().map(|(index, #pattern)| {
                    #(#component_declarations);*

                    (ToString::to_string(&#html), (#(#component_idents),*))
                }).unzip();
                #ident = components;
                html
            }},
            for_loop_components,
            ..Default::default()
        }
    }
}
