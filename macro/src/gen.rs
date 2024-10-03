use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Attribute, Expr, Generics, Ident, Type, Visibility};

use crate::{
    attr::{Argument, Attr},
    expand::{fn_field, impl_field},
};

pub struct Field {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Type,
    pub init: Expr,
}

impl From<impl_field::Field> for Field {
    fn from(value: impl_field::Field) -> Self {
        Self {
            attrs: value.attrs,
            vis: value.vis,
            name: value.name,
            ty: value.ty,
            init: value.init,
        }
    }
}

impl From<fn_field::Field> for Field {
    fn from(value: fn_field::Field) -> Self {
        let name = value.idx.ident();
        Self {
            attrs: value.attrs,
            vis: Visibility::Inherited,
            name,
            ty: value.ty,
            init: value.init,
        }
    }
}

pub struct Gen {
    pub struct_attrs: Vec<Attribute>,
    pub attr: Attr,
    pub ty: Type,
    pub generics: Generics,
    pub fields: Vec<Field>,
}

impl ToTokens for Gen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            struct_attrs,
            attr:
                Attr {
                    vis,
                    constness,
                    new_args: constructor,
                },
            ty,
            generics,
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

        tokens.append_all(quote!(
            #(#struct_attrs)*
            #[repr(Rust)]
            #[non_exhaustive]
            #vis struct #ty {
                #(#field_decl_iter,)*
                __internal_no_default_derive: ::impl_opaque::__private::Opaque,
            } #where_gen

            const _: () = {
                impl #impl_gen ::core::fmt::Debug for #ty #where_gen {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct(::core::stringify!(#ty)).finish_non_exhaustive()
                    }
                }

                impl #impl_gen #ty #where_gen {
                    pub #constness fn new(#constructor) -> Self {
                        Self {
                            #(#field_init_iter,)*
                            #(#constructor_init_iter,)*
                            __internal_no_default_derive: ::impl_opaque::__private::Opaque,
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
            name: &field.name,
            init: &field.init,
        }
    }
}

impl ToTokens for FieldInit<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { name, init } = self;
        tokens.append_all(quote!(#name : #init));
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
            vis: &field.vis,
            name: &field.name,
            ty: &field.ty,
        }
    }
}

impl<'a> From<&'a Argument> for FieldDecl<'a> {
    fn from(field: &'a Argument) -> Self {
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
        tokens.append_all(quote!(#(#attrs)* #vis #name : #ty));
    }
}
