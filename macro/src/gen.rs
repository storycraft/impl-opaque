use proc_macro2::Span;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{Attribute, Generics, Type, Visibility};

use crate::{attr::Constructor, expand::Field};

pub struct Gen {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ty: Type,
    pub generics: Generics,
    pub fields: Vec<Field>,
    pub constructor: Constructor,
}

impl ToTokens for Gen {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self {
            attrs,
            vis,
            ty,
            generics,
            fields,
            constructor,
        } = self;

        let (impl_gen, _, where_gen) = generics.split_for_impl();

        let field_decl_iter = fields.iter().map(FieldDecl::from);
        let field_init_iter = fields.iter().map(|field| &field.init);

        tokens.append_all(quote_spanned!(Span::mixed_site() =>
            #(#attrs)*
            #[repr(Rust)]
            #[non_exhaustive]
            #vis struct #ty (
                #(#field_decl_iter,)*
                ::impl_opaque::__private::Opaque,
            ) #where_gen;

            const _: () = {
                impl #impl_gen ::core::fmt::Debug for #ty #where_gen {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct(::core::stringify!(#ty)).finish_non_exhaustive()
                    }
                }

                impl #impl_gen #ty #where_gen {
                    pub fn new(#constructor) -> Self {
                        Self(#(#field_init_iter,)* ::impl_opaque::__private::Opaque)
                    }
                }
            };
        ));
    }
}

struct FieldDecl<'a> {
    attrs: &'a [Attribute],
    ty: &'a Type,
}

impl<'a> From<&'a Field> for FieldDecl<'a> {
    fn from(field: &'a Field) -> Self {
        FieldDecl {
            attrs: &field.attrs,
            ty: &field.ty,
        }
    }
}

impl ToTokens for FieldDecl<'_> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Self { attrs, ty } = self;
        tokens.append_all(quote_spanned!(Span::mixed_site() => #(#attrs)* #ty));
    }
}
