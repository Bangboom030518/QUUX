#![cfg(test)]
use super::*;
use quote::quote;

#[test]
fn attribute() {
    syn::parse::<Attribute>(quote! {a..1.--::-="VALUE"}.into()).unwrap();
}
