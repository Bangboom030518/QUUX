use super::super::internal::prelude::*;

impl If {
    pub fn html(&self) -> Html {
        let Self {
            condition,
            true_branch,
            false_branch,
        } = self;
        let true_branch = Html::from(*true_branch.clone());
        let false_branch = false_branch.as_ref().map_or_else(
            || quote! { String::new() },
            |item| Html::from(*item.clone()).to_token_stream(),
        );
        // TODO: add components etc.
        Html {
            html: parse_quote! {
                if #condition {
                    #true_branch
                } else {
                    #false_branch
                }
            },
            ..Default::default()
        }
    }
}
