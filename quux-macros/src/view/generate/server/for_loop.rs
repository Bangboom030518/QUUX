use super::super::internal::prelude::*;
use crate::view::parse::prelude::*;

impl ForLoop {
    pub fn html(self) -> Html {
        let ident = self.ident();
        let Self {
            pattern,
            iterable,
            mut item,
            ..
        } = self.clone();
        let iterable = match iterable {
            ForLoopIterable::Static(iterable) => quote! { #iterable },
            ForLoopIterable::Reactive(iterable) => {
                item.insert_for_loop_id(self.id);
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

        // push the the type of for loop's children to the return type so the root item may declare and use them
        for_loop_components.0.push(self);

        let declarations = components.declarations();
        let expr = components.expr();

        Html {
            html: parse_quote! {{
                let (html, components): (String, Vec<_>) = (#iterable).enumerate().map(|(index, #pattern)| {
                    #declarations

                    (ToString::to_string(&#html), #expr)
                }).unzip();
                #ident = components;
                html
            }},
            for_loop_components,
            ..Default::default()
        }
    }
}
