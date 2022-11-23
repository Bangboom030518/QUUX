use proc_macro::TokenStream;
use syn::parse_macro_input;

mod view;

/// $ident:tag_name ( $( $ident:key = $expr:value )* ) {
///     (!($$ $ident:reactive_store) $self)* | $$ $ident:reactive_store
/// }
///
/// { $expr:content }
///
/// $$ $ident:reactive_store

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let tree = parse_macro_input!(input as view::Element);
    view::generate(tree).into()
}
