use super::parse::Element;
use proc_macro2::TokenStream;
use quote::quote;

mod client;
mod server;

pub fn generate(tree: Element) -> TokenStream {
    let server = server::generate(&tree);
    let client = client::generate(&tree);
    quote! {
        shared::cfg_if::cfg_if! {
            if #[cfg(target_arch = "wasm32")] {
                {#client}
            } else {
                {#server}
            }
        }
    }
}
