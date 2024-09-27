use proc_macro2::Span;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{Attribute, Expr, Generics, Ident, Type, Visibility};

use crate::{
    attr::{Constructor, ConstructorArgs},
    expand::Field,
};

pub struct Gen {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ty: Type,
    pub generics: Generics,
    pub constructor: Constructor,
    pub fields: Vec<Field>,
}

impl ToTokens for Gen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            attrs,
            vis,
            ty,
            generics,
            constructor,
            fields,
        } = self;

        let (impl_gen, _, where_gen) = generics.split_for_impl();

        let field_decl_iter = constructor
            .iter()
            .filter(|arg| arg.vis.is_some())
            .map(FieldDecl::from)
            .chain(fields.iter().map(FieldDecl::from));

        let constructor_init_iter = constructor
            .iter()
            .filter(|arg| arg.vis.is_some())
            .map(|arg| &arg.pat.ident);
        let field_init_iter = fields.iter().map(FieldInit::from);

        tokens.append_all(quote_spanned!(Span::mixed_site() =>
            #(#attrs)*
            #[repr(Rust)]
            #[non_exhaustive]
            #vis struct #ty {
                #(#field_decl_iter,)*
            } #where_gen

            const _: () = {
                impl #impl_gen ::core::fmt::Debug for #ty #where_gen {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct(::core::stringify!(#ty)).finish_non_exhaustive()
                    }
                }

                impl ::core::default::Default for #ty where for<'a> *const &'a #ty: ::core::marker::Send {
                    fn default() -> Self {
                        ::core::unreachable!()
                    }
                }

                impl #impl_gen #ty #where_gen {
                    pub fn new(#constructor) -> Self {
                        Self {
                            #(#field_init_iter,)*
                            #(#constructor_init_iter,)*
                        }
                    }
                }
            };
        ));
    }
}

struct FieldInit<'a> {
    name: &'a Ident,
    init: &'a Expr,
}

impl<'a> From<&'a Field> for FieldInit<'a> {
    fn from(field: &'a Field) -> Self {
        Self {
            name: &field.index,
            init: &field.init,
        }
    }
}

impl ToTokens for FieldInit<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { name, init } = self;
        tokens.append_all(quote_spanned!(Span::mixed_site() => #name : #init));
    }
}

struct FieldDecl<'a> {
    attrs: &'a [Attribute],
    vis: &'a Visibility,
    name: &'a Ident,
    ty: &'a Type,
}

impl<'a> From<&'a Field> for FieldDecl<'a> {
    fn from(field: &'a Field) -> Self {
        Self {
            attrs: &field.attrs,
            vis: &Visibility::Inherited,
            name: &field.index,
            ty: &field.ty,
        }
    }
}

impl<'a> From<&'a ConstructorArgs> for FieldDecl<'a> {
    fn from(field: &'a ConstructorArgs) -> Self {
        Self {
            attrs: &[],
            vis: field.vis.as_ref().unwrap_or(&Visibility::Inherited),
            name: &field.pat.ident,
            ty: &field.ty,
        }
    }
}

impl ToTokens for FieldDecl<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            attrs,
            vis,
            name,
            ty,
        } = self;
        tokens.append_all(quote_spanned!(Span::mixed_site() => #(#attrs)* #vis #name : #ty));
    }
}
