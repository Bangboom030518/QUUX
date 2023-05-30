use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Token, Type,
};

struct Route {
    ty: Type,
    variant_name: Ident,
}

impl Route {
    fn variant(&self) -> TokenStream {
        let Self { ty, variant_name } = self;
        quote! {
            #variant_name(#ty)
        }
    }

    fn implementations(&self) -> TokenStream {
        let Self { ty, variant_name } = self;

        quote! {
            impl From<#ty> for Routes {
                fn from(value: #ty) -> Self {
                    Self::#variant_name(value)
                }
            }
        }
    }
}

struct Routes(Vec<Route>);

impl Routes {
    fn enum_declaration(&self) -> TokenStream {
        let variants = self.0.iter().map(Route::variant);
        quote! {
            #[derive(quux::prelude::Serialize, quux::prelude::Deserialize, Clone)]
            pub enum Routes {
                #(#variants),*
            }
        }
    }

    fn implementations(&self) -> TokenStream {
        let (implementations, variants): (TokenStream, Vec<_>) = self
            .0
            .iter()
            .map(|route| (route.implementations(), &route.variant_name))
            .unzip();

        quote! {
            #implementations

            impl quux::component::Routes for Routes {
                #[quux::prelude::client]
                fn hydrate(self) {
                    match self {
                        #(Self::#variants(component) => {
                            let mut tree = quux::component::Component::render(component, quux::context::Context::new());
                            quux::tree::Item::insert_id(&mut tree, 0);
                            // quux::dom::console_log!("{:#?}", tree);
                            quux::tree::Item::hydrate(&mut tree);
                        }),*
                    };
                }
            }
        }
    }
}

impl Parse for Routes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let routes = Punctuated::<_, Token![,]>::parse_terminated(input)?
            .into_iter()
            .enumerate()
            .map(|(index, ty)| Route {
                variant_name: format_ident!("Route{index}"),
                ty,
            })
            .collect();

        Ok(Self(routes))
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<Routes> for TokenStream {
    fn from(value: Routes) -> Self {
        let enum_declaration = value.enum_declaration();
        let implementations = value.implementations();
        let tokens = quote! {
            #enum_declaration

            #implementations
        };
        tokens
    }
}

pub fn routes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let routes = parse_macro_input!(input as Routes);
    TokenStream::from(routes).into()
}
