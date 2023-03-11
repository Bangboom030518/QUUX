use generate::generate;
use parse::prelude::View;
use proc_macro::TokenStream;
use syn::parse_macro_input;
mod generate;
mod parse;

pub fn view(input: TokenStream) -> TokenStream {
    let tree = parse_macro_input!(input as View);
    generate(&tree).into()
}
