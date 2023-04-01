use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, punctuated::Punctuated, Token, Type};

struct Route {
    ty: Type,
    variant_name: Ident,
}

impl Route {
    fn variant(&self) -> TokenStream {
        let Self { ty, variant_name } = self;
        quote! {
            #variant_name(quux::view::SerializedComponent<#ty>)
        }
    }

    fn implementations(&self) -> TokenStream {
        let Self { ty, variant_name } = self;
        quote! {
            impl From<SerializedComponent<#ty>> for Routes {
                fn from(value: SerializedComponent<#ty>) -> Self {
                    Self::#variant_name(value)
                }
            }
        }
    }
}

struct Routes(Vec<Route>, bool);

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
                fn render(self) {
                    match self {
                        #(Self::#variants(component) => {
                            quux::view::SerializedComponent::render(component);
                        }),*
                    };
                }
            }
        }
    }
}

impl Parse for Routes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let magic = input.parse::<Token![$]>().is_ok();

        let components = Punctuated::<_, Token![,]>::parse_terminated(input)?
            .into_iter()
            .enumerate()
            .map(|(index, ty)| Route {
                variant_name: format_ident!("Route{index}"),
                ty,
            })
            .collect();

        Ok(Self(components, magic))
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
        if value.1 {
            std::fs::write("magic.rs", tokens.to_string()).unwrap();
        }
        tokens
    }
}

pub fn routes(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let routes = parse_macro_input!(input as Routes);
    TokenStream::from(routes).into()
}

/*
#[derive(Serialize, Deserialize)]
pub enum Routes {
    Set(SerializedComponent<pages::Set>),
    ServerError(SerializedComponent<pages::Error>),
    Create(SerializedComponent<pages::Create>),
}

impl From<SerializedComponent<pages::Set>> for Routes {
    fn from(value: SerializedComponent<pages::Set>) -> Self {
        Self::Set(value)
    }
}

impl From<SerializedComponent<pages::Error>> for Routes {
    fn from(value: SerializedComponent<pages::Error>) -> Self {
        Self::ServerError(value)
    }
}

impl From<SerializedComponent<pages::Create>> for Routes {
    fn from(value: SerializedComponent<pages::Create>) -> Self {
        Self::Create(value)
    }
}

impl quux::component::Routes for Routes {
    #[client]
    fn render(self) {
        match self {
            Self::Set(set) => {
                set.render();
            }
            Self::ServerError(server_error) => {
                server_error.render();
            }
            Self::Create(create) => {
                create.render();
            }
        };
    }
}
*/
