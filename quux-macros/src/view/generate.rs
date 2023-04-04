// TODO: component:0 { component:1 { element:1.1 } } element:0.1
use internal::prelude::*;
pub use server::Html;

mod client;
mod server;
mod declarations;

pub fn generate(tree: &View) -> TokenStream {
    let server::Output {
        render_output: server,
        client_context,
    } = server::generate(tree);
    let client = client::generate(tree);
    let tokens = quote! {
        {
            #client_context;
            quux::cfg_if::cfg_if! {
                if #[cfg(target_arch = "wasm32")] {
                    {#client}
                } else {
                    {#server}
                }
            }
        }
    };
    // if tree.element.attributes.attributes.contains_key("magic") {
        std::fs::write(
            "expansion.rs",
            quote! {
                fn main() {
                    #tokens
                }
            }.to_string()
        ).unwrap();
    // }
    tokens
}

mod internal {
    pub mod prelude {
        pub use super::super::{declarations::prelude::*, Html};
        pub use crate::view::parse::prelude::*;
        pub use proc_macro2::{Ident, TokenStream};
        pub use quote::{format_ident, quote, ToTokens};
        pub use syn::{parse_quote, Expr, Type, TypePath};
    }
}
