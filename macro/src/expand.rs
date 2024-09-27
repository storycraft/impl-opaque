use quote::{format_ident, quote_spanned};
use syn::{
    spanned::Spanned, visit::Visit, visit_mut::VisitMut, Attribute, Block, Expr, Ident, Local,
    LocalInit, PatType, Token, Type,
};

pub struct Field {
    pub attrs: Vec<Attribute>,
    pub index: Ident,
    pub ty: Type,
    pub init: Expr,
}

pub struct FieldExpander {
    pub fields: Vec<Field>,
}

impl FieldExpander {
    pub const fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn expand(&mut self, block: &mut Block) {
        Expander(&mut self.fields).visit_block_mut(block);
    }
}

struct Expander<'a>(&'a mut Vec<Field>);

impl VisitMut for Expander<'_> {
    fn visit_local_mut(&mut self, local: &mut syn::Local) {
        let Some(field_pos) = local
            .attrs
            .iter()
            .position(|attr| attr.path().is_ident("field"))
        else {
            return;
        };

        let Some(ty) = LocalVisitor::find(local) else {
            local.init = Some(LocalInit {
                eq_token: Token![=](local.span()),
                expr: Box::new(Expr::Verbatim(quote_spanned!(local.span() =>
                    ::core::compile_error!("Field must have a type")
                ))),
                diverge: None,
            });

            return;
        };

        let index = format_ident!("__{}", self.0.len());

        let init: Expr = {
            let replaced = LocalInit {
                eq_token: Token![=](local.span()),
                expr: Box::new(Expr::Verbatim({
                    quote_spanned!(local.span() => self.#index)
                })),
                diverge: None,
            };

            match local.init.replace(replaced) {
                None => Expr::Verbatim(
                    quote_spanned!(local.span() => ::core::compile_error!("Field does not have a initializer")),
                ),

                Some(LocalInit {
                    diverge: Some(_), ..
                }) => Expr::Verbatim(
                    quote_spanned!(local.span() => ::core::compile_error!("Field cannot diverge")),
                ),

                Some(LocalInit { expr, .. }) => *expr,
            }
        };

        self.0.push(Field {
            attrs: local.attrs.drain(field_pos..).skip(1).collect(),
            index,
            ty,
            init,
        });
    }
}

struct LocalVisitor {
    pub ty: Option<Type>,
}

impl LocalVisitor {
    pub fn find(local: &Local) -> Option<Type> {
        let mut this = Self { ty: None };
        this.visit_local(local);

        this.ty
    }
}

impl Visit<'_> for LocalVisitor {
    fn visit_pat_type(&mut self, i: &PatType) {
        self.visit_pat(&i.pat);

        self.ty = Some(Type::clone(&i.ty));
    }
}
