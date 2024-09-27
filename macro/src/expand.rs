use proc_macro2::Span;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    visit::Visit, visit_mut::VisitMut, Attribute, Block, Expr, Ident, Local, LocalInit, PatType,
    Stmt, Token, Type,
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
    fn visit_stmt_mut(&mut self, i: &mut Stmt) {
        let Stmt::Local(local) = i else { return };

        let Some(field_pos) = local
            .attrs
            .iter()
            .position(|attr| attr.path().is_ident("field"))
        else {
            return;
        };

        let Some(ty) = LocalVisitor::find(local) else {
            *i = Stmt::Expr(
                Expr::Verbatim(quote!(::core::compile_error!("Field must have type"))),
                Some(Default::default()),
            );

            return;
        };

        let index = format_ident!("__{}", self.0.len());

        let init: Expr = {
            let replaced = LocalInit {
                eq_token: Token![=](Span::mixed_site()),
                expr: Box::new(Expr::Verbatim({
                    quote_spanned!(Span::call_site() => self.#index)
                })),
                diverge: None,
            };

            match local.init.replace(replaced) {
                None => Expr::Verbatim(
                    quote! { ::core::compile_error!("Field does not have a initializer") },
                ),

                Some(LocalInit {
                    diverge: Some(_), ..
                }) => Expr::Verbatim(quote! { ::core::compile_error!("Field cannot diverge") }),

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
