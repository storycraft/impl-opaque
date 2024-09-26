use proc_macro2::Span;
use quote::{quote, quote_spanned};
use syn::{
    visit::Visit, visit_mut::VisitMut, Attribute, Block, Expr, Index, Local, LocalInit, PatType, Stmt, Token, Type, TypeInfer
};

pub struct Field {
    pub attrs: Vec<Attribute>,
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

        let ty = LocalTyVisitor::find(local).unwrap_or_else(|| {
            Type::Infer(TypeInfer {
                underscore_token: Token![_](Span::mixed_site()),
            })
        });

        let init: Expr = {
            let replaced = LocalInit {
                eq_token: Token![=](Span::mixed_site()),
                expr: Box::new(Expr::Verbatim({
                    let index = Index::from(self.0.len());
                    quote_spanned!(Span::call_site() => self.#index)
                })),
                diverge: None,
            };

            match local.init.replace(replaced) {
                None => Expr::Verbatim(
                    quote! { ::core::compile_error!("Field {} does not have a initializer") },
                ),

                Some(LocalInit {
                    diverge: Some(_), ..
                }) => Expr::Verbatim(quote! { ::core::compile_error!("Field cannot diverge") }),

                Some(LocalInit { expr, .. }) => *expr,
            }
        };

        self.0.push(Field {
            attrs: local.attrs.drain(field_pos..).skip(1).collect(),
            ty,
            init,
        });
    }
}

struct LocalTyVisitor {
    pub ty: Option<Type>,
}

impl LocalTyVisitor {
    pub fn find(local: &Local) -> Option<Type> {
        let mut this = Self { ty: None };
        this.visit_local(local);

        this.ty
    }
}

impl Visit<'_> for LocalTyVisitor {
    fn visit_pat_type(&mut self, i: &PatType) {
        self.visit_pat(&i.pat);

        self.ty = Some(Type::clone(&i.ty));
    }
}
