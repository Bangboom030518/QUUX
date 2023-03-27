use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, parse_quote, punctuated::Punctuated, Token, Type};

struct Component {
    ty: Type,
    variant_name: Ident,
}

impl Component {
    fn variant(&self) -> TokenStream {
        let Self { ty, variant_name } = self;
        quote! {
            #variant_name(#ty)
        }
    }

    fn implementations(&self) -> TokenStream {
        let Self { ty, variant_name } = self;
        quote! {
            impl From<#ty> for ComponentEnum {
                fn from(value: #ty) -> Self {
                    Self::#variant_name(value)
                }
            }

            impl TryFrom<ComponentEnum> for #ty {
                type Error = ();

                fn try_from(value: ComponentEnum) -> Result<Self, Self::Error> {
                    if let ComponentEnum::#variant_name(component) = value {
                        Ok(component)
                    } else {
                        Err(())
                    }
                }
            }
        }
    }
}

struct Components(Vec<Component>, bool);

impl Components {
    fn enum_declaration(&self) -> TokenStream {
        let variants = self.0.iter().map(Component::variant);
        quote! {
            #[derive(Serialize, Deserialize, Clone, Debug)]
            pub enum ComponentEnum {
                #(#variants),*
            }
        }
    }

    fn implementations(&self) -> TokenStream {
        let (implementations, variants): (Vec<_>, Vec<_>) = self
            .0
            .iter()
            .map(|component| (component.implementations(), &component.variant_name))
            .unzip();
        quote! {
            impl quux::component::Enum for ComponentEnum {
                fn render(self, context: render::Context<Self>) -> quux::component::EnumRenderOutput<Self> {
                    match self {
                        #(Self::#variants(component) => component.render(context).into()),*
                    }
                }
            }

            #(#implementations)*
        }
    }
}

impl Parse for Components {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let magic = input.parse::<Token![$]>().is_ok();

        let components = Punctuated::<_, Token![,]>::parse_terminated(input)?
            .into_iter()
            .chain(std::iter::once(parse_quote! {
                quux::initialisation_script::InitialisationScript<ComponentEnum>
            }))
            .enumerate()
            .map(|(index, ty)| Component {
                variant_name: format_ident!("Component{index}"),
                ty,
            })
            .collect();

        Ok(Self(components, magic))
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<Components> for TokenStream {
    fn from(value: Components) -> Self {
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

pub fn init_components(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let components = parse_macro_input!(input as Components);
    TokenStream::from(components).into()
}
