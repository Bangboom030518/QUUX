// TODO: component:0 { component:1 { element:1.1 } } element:0.1
pub use server::Html;
use internal::prelude::*;

mod client;
mod server;


pub fn generate(tree: &View) -> TokenStream {
    let server = server::generate(tree);
    let client = client::generate(tree);
    quote! {
        quux::cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                {#client}
            } else {
                {#server}
            }
        }
    }
}

mod internal {
    pub mod prelude {
        pub use proc_macro2::TokenStream;
        pub use quote::{quote, format_ident, ToTokens};
        pub use syn::{parse_quote, Expr};
        pub use crate::view::parse::prelude::*;
        pub use super::super::Html;
    }
}
