use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{parse::Parse, parse_macro_input, parse_quote, Token, Type};

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

struct Components(Vec<Component>);

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
                fn render(&self, context: render::Context<Self>) -> render::Output<Self> {
                    match self {
                        #(Self::#variants(component) => component.render(context)),*
                    }
                }
            }

            #(#implementations)*
        }
    }
}

impl Parse for Components {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut components = vec![Component {
            ty: parse_quote! {
                quux::quux_initialise::QUUXInitialise<ComponentEnum>
            },
            variant_name: format_ident!("QUUXInitialise"),
        }];
        let mut index = 0;
        loop {
            components.push(Component {
                ty: input.parse()?,
                variant_name: format_ident!("Component{index}"),
            });
            if input.is_empty() {
                break;
            }
            index += 1;
            input.parse::<Token![,]>()?;
        }
        Ok(Self(components))
    }
}

impl From<Components> for TokenStream {
    fn from(value: Components) -> Self {
        let enum_declaration = value.enum_declaration();
        let implementations = value.implementations();
        quote! {
            #enum_declaration

            #implementations
        }
    }
}

pub fn init_components(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let components = parse_macro_input!(input as Components);
    TokenStream::from(components).into()
}
