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

    fn implementations(&self, warp: bool) -> TokenStream {
        let Self { ty, variant_name } = self;
        let warp_code = warp
            .then(|| {
                quote! {
                    #[quux::prelude::server]
                    impl quux::warp::Reply for #ty {
                        fn into_response(self) -> warp::reply::Response {
                            warp::reply::html(Routes::render_to_string(self)).into_response()
                        }
                    }
                }
            })
            .unwrap_or_default();
        quote! {
            #warp_code

            impl From<#ty> for Routes {
                fn from(value: #ty) -> Self {
                    Self::#variant_name(value)
                }
            }
        }
    }
}

struct Routes {
    routes: Vec<Route>,
    warp: bool,
}

impl Routes {
    fn enum_declaration(&self) -> TokenStream {
        let variants = self.routes.iter().map(Route::variant);
        quote! {
            #[derive(quux::prelude::Serialize, quux::prelude::Deserialize, Clone)]
            pub enum Routes {
                #(#variants),*
            }
        }
    }

    fn implementations(&self) -> TokenStream {
        let (implementations, variants): (TokenStream, Vec<_>) = self
            .routes
            .iter()
            .map(|route| (route.implementations(self.warp), &route.variant_name))
            .unzip();

        quote! {
            #implementations

            impl quux::component::Routes for Routes {
                #[quux::prelude::client]
                fn hydrate(self) {
                    match self {
                        #(Self::#variants(component) => {
                            quux::tree::Hydrate::hydrate(quux::component::Component::render(component, quux::context::Context::new()));
                        }),*
                    };
                }
            }
        }
    }
}

fn include_warp(input: &mut ParseStream) -> bool {
    if input.parse::<Token![#]>().is_err() {
        return false;
    }
    if let Ok(ident) = input.parse::<Ident>() {
        return ident == "warp";
    }
    false
}

impl Parse for Routes {
    fn parse(mut input: ParseStream) -> syn::Result<Self> {
        let warp = include_warp(&mut input);
        let routes = Punctuated::<_, Token![,]>::parse_terminated(input)?
            .into_iter()
            .enumerate()
            .map(|(index, ty)| Route {
                variant_name: format_ident!("Route{index}"),
                ty,
            })
            .collect();

        Ok(Self { routes, warp })
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
