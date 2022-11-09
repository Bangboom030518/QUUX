use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use std::fs;
use syn::{parse_macro_input, DeriveInput};

const LOG_PATH: &str = "./log";

enum Item {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

struct ItemStruct {
    struct_token: Token![struct],
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            input.parse().map(Item::Struct)
        } else if lookahead.peek(Token![enum]) {
            input.parse().map(Item::Enum)
        } else {
            Err(lookahead.error())
        }
    }
}

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(ItemStruct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}

/// $ident:tag_name ( $( $ident:key = $expr:value )* ) { 
///     (!($$ $ident:reactive_store) $self)* | $$ $ident:reactive_store
/// }
///
/// { $expr:content }
///
/// $$ $ident:reactive_store

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    fs::write(LOG_PATH, "").unwrap();
    for token in input {
        // log(&format!("{:?}\n", token))
    }
    let tree = parse_macro_input!(input);
    
    log(&format!("{:?}", tree));
    quote! {
        println!("Hello!")
    }
    .into()
}

#[test]
fn test_html() {
    html(quote! {
        button(class="btn") {
            { "Click Me" }
        }
    }.into());
}

#[inline]
fn log(append_str: &str) {
    let contents = fs::read_to_string(LOG_PATH).unwrap();
    fs::write(LOG_PATH, contents + append_str).unwrap();
}
