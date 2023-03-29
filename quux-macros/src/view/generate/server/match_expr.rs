use super::super::internal::prelude::*;
use match_expr::Arm;

impl Match {
    pub fn html(&self) -> Html {
        let Self { scrutinee, arms } = &self;
        let arms: TokenStream = arms
            .into_iter()
            .map(|Arm { pattern, item }| {
                let item = Html::from(item.clone());
                quote! {
                    #pattern => {
                        #item
                    },
                }
            })
            .collect();
        // TODO: add components etc.
        Html {
            html: parse_quote! {
                match #scrutinee {
                    #arms
                }
            },
            ..Default::default()
        }
    }
}
