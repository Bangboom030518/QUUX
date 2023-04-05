use super::internal::prelude::*;

pub trait Declaration {
    fn ty(&self) -> Type;
    fn ident(&self) -> Ident;

    fn declaration(&self) -> syn::Stmt {
        let ty = self.ty();
        let ident = self.ident();

        parse_quote! {
            let #ident: #ty;
        }
    }
}

impl Declaration for ForLoop {
    fn ty(&self) -> Type {
        let item = self.item.clone();
        let Html { components, .. } = (*item).into();

        let ty = components.ty();
        parse_quote! { Vec<#ty> }
    }
    fn ident(&self) -> Ident {
        let id = self.id;
        format_ident!("for_loop_components_{id}")
    }
}

impl Declaration for Component {
    fn ty(&self) -> Type {
        let ty = &self.name;
        parse_quote! {
            quux::view::SerializedComponent<#ty>
        }
    }

    fn ident(&self) -> Ident {
        self.ident.clone()
    }
}
#[derive(Clone, Default)]
pub struct ForLoops(pub Vec<ForLoop>);

impl Declarations for ForLoops {
    type Item = ForLoop;

    fn items(&self) -> &[Self::Item] {
        &self.0
    }
}

#[derive(Clone, Default)]
pub struct Components(pub Vec<Component>);

impl Declarations for Components {
    type Item = Component;

    fn items(&self) -> &[Self::Item] {
        &self.0
    }
}

impl Components {
    pub fn bindings(&self) -> Vec<&Ident> {
        self.0
            .iter()
            .filter_map(|component| component.binding.as_ref())
            .collect()
    }

    pub fn types(&self) -> Vec<&syn::Path> {
        self.0.iter().map(|component| &component.name).collect()
    }
}

pub trait Declarations {
    type Item: Declaration;

    fn items(&self) -> &[Self::Item];

    fn ty(&self) -> Type {
        let items = self.items();
        let types = items.iter().map(Declaration::ty);
        parse_quote! {
            (#(#types,)*)
        }
    }

    fn expr(&self) -> Expr {
        let items = self.items();
        let idents = items.iter().map(Declaration::ident);

        parse_quote! {
            (#(#idents,)*)
        }
    }

    fn declarations(&self) -> TokenStream {
        let items = self.items();
        let declarations = items.iter().map(Declaration::declaration);
        quote! {
            #(#declarations;)*
        }
    }
}

pub mod prelude {
    pub use super::{Components, Declaration, Declarations, ForLoops};
}
