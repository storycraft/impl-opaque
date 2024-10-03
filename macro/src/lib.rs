mod attr;
mod expand;
mod gen;

use attr::Attr;
use expand::{fn_field::FnFieldExpander, impl_field::ImplFieldExpander};
use gen::{Field, Gen};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ImplItemFn, ItemImpl};

#[proc_macro_attribute]
pub fn opaque(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: Attr = parse_macro_input!(attr);
    let mut block: ItemImpl = parse_macro_input!(item);

    let impl_iter = {
        let mut impl_expand = ImplFieldExpander::new();
        impl_expand.expand(&mut block);

        impl_expand.fields.into_iter().map(Into::<Field>::into)
    };

    let fn_iter = {
        let mut fn_expand = FnFieldExpander::new();
        for item in &mut block.items {
            if let ImplItem::Fn(ImplItemFn { block, .. }) = item {
                fn_expand.expand(block);
            }
        }

        fn_expand.fields.into_iter().map(Into::<Field>::into)
    };

    let gen = Gen {
        struct_attrs: block.attrs.drain(..).collect(),
        attr,
        ty: *block.self_ty.clone(),
        generics: block.generics.clone(),
        fields: impl_iter.chain(fn_iter).collect(),
    };

    TokenStream::from(quote!(
        #gen
        #block
    ))
}
