#![warn(clippy::pedantic, clippy::nursery)]
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod view;

// TODO: document
#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let tree = parse_macro_input!(input as view::Element);
    view::generate(&tree).into()
}
