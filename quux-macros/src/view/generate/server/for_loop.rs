use super::super::internal::prelude::*;
use crate::view::parse::prelude::*;
use for_loop::Iterable;

impl ForLoop {
    pub fn html(self) -> Html {
        let ident: Ident = self.ident();
        let Self {
            pattern,
            iterable,
            mut item,
            ..
        } = self.clone();
        let iterable = match iterable {
            Iterable::Static(iterable) => quote! { #iterable },
            Iterable::Reactive(iterable) => {
                item.insert_for_loop_id(self.id);
                quote! {
                    (std::cell::Ref::<Vec<_>>::from(&#iterable)).iter().cloned()
                }
            }
        };
        let Html { html, ty } = (*item).into();

        Html::new(
            parse_quote! {{
                let html: String = (#iterable).enumerate().map(|(index, #pattern)| {
                    ToString::to_string(&#html)
                }).collect();
                #ident = components;
                html
            }},
            todo!(),
        )
    }
}

impl From<ForLoop> for Html {
    fn from(value: ForLoop) -> Self {
        let ident: Ident = value.ident();
        let ForLoop {
            pattern,
            iterable,
            mut item,
            ..
        } = value;
        let iterable = match iterable {
            Iterable::Static(iterable) => quote! { #iterable },
            Iterable::Reactive(iterable) => {
                item.insert_for_loop_id(value.id);
                quote! {
                    (std::cell::Ref::<Vec<_>>::from(&#iterable)).iter().cloned()
                }
            }
        };
        // let Html {
        //     html,
        //     components,
        //     mut for_loop_components,
        // } = (*item).into();

        // let declarations = components.declarations();
        // let expr = components.expr();

        Html::new(
            parse_quote! {{
                let html: String = (#iterable).enumerate().map(|(index, #pattern)| {
                    ToString::to_string(&#item)
                }).collect();
                #ident = components;
                html
            }},
            todo!(),
        )
    }
}
