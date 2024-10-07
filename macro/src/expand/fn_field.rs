use core::mem;

use alloc::{boxed::Box, vec::Vec};
use quote::{format_ident, quote_spanned};
use syn::{
    spanned::Spanned, visit::Visit, visit_mut::VisitMut, Attribute, Block, Expr, Ident, Index,
    Local, LocalInit, PatType, Token, Type,
};

pub struct IndexIdent(pub Index);
impl IndexIdent {
    pub fn ident(&self) -> Ident {
        format_ident!("__internal_do_not_use_or_you_will_be_fired_{}", self.0)
    }
}

pub struct Field {
    pub attrs: Vec<Attribute>,
    pub idx: IndexIdent,
    pub ty: Type,
    pub init: Expr,
}

pub struct FnFieldExpander {
    pub fields: Vec<Field>,
}

impl FnFieldExpander {
    pub const fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn expand(&mut self, block: &mut Block) {
        Visitor(&mut self.fields).visit_block_mut(block);
    }
}

struct Visitor<'a>(&'a mut Vec<Field>);

impl VisitMut for Visitor<'_> {
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

        let idx = IndexIdent(Index::from(self.0.len()));

        let init = {
            match local.init {
                None => {
                    local.init = Some(LocalInit {
                        eq_token: Token![=](local.span()),
                        expr: Box::new(Expr::Verbatim(
                            quote_spanned!(local.span() => ::core::compile_error!("field does not have a initializer")),
                        )),
                        diverge: None,
                    });

                    return;
                }

                Some(LocalInit { ref mut expr, .. }) => {
                    let span = expr.span();
                    let ident = idx.ident();
                    *mem::replace(
                        expr,
                        Box::new(Expr::Verbatim(quote_spanned!(span => self.#ident))),
                    )
                }
            }
        };

        self.0.push(Field {
            attrs: local.attrs.drain(field_pos..).skip(1).collect(),
            idx,
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
