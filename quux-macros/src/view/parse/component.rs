use super::internal::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use syn::parse_quote;

static ID: AtomicU64 = AtomicU64::new(0);

#[derive(Clone)]
pub struct Component {
    pub name: Type,
    pub props: Props,
    pub binding: Option<Ident>,
    /// Will be updated with a for loop id if this component is used in a for loop
    pub for_loop_id: Option<u64>,
    pub ident: Ident,
}

impl Parse for Component {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let props = input.parse()?;

        let binding = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Some(input.parse()?)
        } else {
            None
        };

        Ok(Self {
            name,
            props,
            binding,
            for_loop_id: None,
            ident: quote::format_ident!("component_{}", ID.fetch_add(1, Relaxed)),
        })
    }
}

#[derive(Clone)]
pub struct Props(pub Expr);

impl ToTokens for Props {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl Parse for Props {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let props = if input.peek(Paren) {
            let attributes_buffer;
            parenthesized!(attributes_buffer in input);
            attributes_buffer.parse()?
        } else {
            parse_quote! {
                ()
            }
        };
        Ok(Self(props))
    }
}
