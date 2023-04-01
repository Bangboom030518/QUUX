#![warn(clippy::pedantic, clippy::nursery)]
#![feature(exact_size_is_empty, iter_intersperse)]
use proc_macro::TokenStream;
use quote::quote;

mod routes;
mod view;

// TODO: document
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    view::view(input)
}

#[proc_macro]
pub fn routes(input: TokenStream) -> TokenStream {
    routes::routes(input)
}

/// Includes the item only on the server build. Equivalent to `#[cfg(not(target_arch = "wasm32"))]`
#[proc_macro_attribute]
pub fn server(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    quote! {
        #[cfg(not(target_arch = "wasm32"))]
        #item
    }
    .into()
}
/// Includes the item only on the client build. Equivalent to `#[cfg(target_arch = "wasm32")]`
#[proc_macro_attribute]
pub fn client(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    let item = proc_macro2::TokenStream::from(item);
    quote! {
        #[cfg(target_arch = "wasm32")]
        #item
    }
    .into()
}
