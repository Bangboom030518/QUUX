use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use std::fs;

const LOG_PATH: &str = "./log";

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    fs::write(LOG_PATH, "").unwrap();
    for token in input {
        // match token {
        //     TokenTree::Punct(value) => {
        //         let x = value;
        //         match value.as_char() {
        //             '<' =>
        //         }
        //     },
        // }
        log(&format!("{:?}\n", token))
    }
    quote! {
        println!("Hello!")
    }
    .into()
}

#[inline]
fn log(append_str: &str) {
    let contents = fs::read_to_string(LOG_PATH).unwrap();
    fs::write(LOG_PATH, contents + append_str).unwrap();
}
