use proc_macro::{TokenStream, TokenTree};

/// Used to constuct an element tree in a JSX-like manner
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    TokenStream::new()
}

// Used to define an element type, and implement the nescessary traits
#[proc_macro]
pub fn element(input: TokenStream) -> TokenStream {
    let name = match input.into_iter().next().expect("Expected Token") {
        TokenTree::Ident(name) => name,
        _ => panic!("Expected Identifier"),
    };

    let name = name.to_string();

    let tag_name = name.to_string().to_lowercase();

    quote::quote! {
        #[derive(Clone, Default)]
        pub struct #name <'a> {
            pub children: Vec<&'a dyn Element>,
            pub attributes: HashMap<String, String>,
        }

        impl<'a> Element for #name <'a> {
            fn get_tag_name(&self) -> &'static str {
                #tag_name
            }

            fn get_attributes(&self) -> HashMap<String, String> {
                self.attributes.clone()
            }

            fn get_children(&self) -> &[&dyn Element] {
                &self.children
            }
        }
    }
    .into()
}
