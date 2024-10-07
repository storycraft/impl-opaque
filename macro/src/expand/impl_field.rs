use alloc::vec::Vec;
use core::mem;
use syn::{
    parse::{Parse, ParseStream},
    visit_mut::VisitMut,
    Attribute, Expr, Ident, ImplItem, ItemImpl, Token, Type, Visibility,
};

pub struct Field {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Type,
    pub init: Expr,
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            attrs: Vec::new(),
            vis: input.parse()?,
            name: input.parse()?,
            ty: {
                input.parse::<Token![:]>()?;
                input.parse()?
            },
            init: {
                input.parse::<Token![=]>()?;
                input.parse()?
            },
        })
    }
}

pub struct ImplFieldExpander {
    pub fields: Vec<Field>,
}

impl ImplFieldExpander {
    pub const fn new() -> Self {
        Self { fields: Vec::new() }
    }

    pub fn expand(&mut self, item_impl: &mut ItemImpl) {
        Visitor(&mut self.fields).visit_item_impl_mut(item_impl);
    }
}

struct Visitor<'a>(&'a mut Vec<Field>);

impl VisitMut for Visitor<'_> {
    fn visit_item_impl_mut(&mut self, i: &mut syn::ItemImpl) {
        i.items.retain_mut(|item| {
            let macro_item = match item {
                ImplItem::Macro(item) if item.mac.path.is_ident("field") => item,
                _ => return true,
            };

            let mut field = match macro_item.mac.parse_body::<Field>() {
                Ok(field) => field,

                Err(err) => {
                    *item = ImplItem::Verbatim(err.into_compile_error());
                    return true;
                }
            };
            mem::swap(&mut field.attrs, &mut macro_item.attrs);
            self.0.push(field);

            false
        });
    }
}
