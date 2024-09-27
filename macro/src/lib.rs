mod attr;
mod expand;
mod gen;

use attr::Attr;
use expand::FieldExpander;
use gen::Gen;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ImplItem, ImplItemFn, ItemImpl};

#[proc_macro_attribute]
pub fn opaque(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr: Attr = parse_macro_input!(attr);
    let mut block: ItemImpl = parse_macro_input!(item);

    let mut expander = FieldExpander::new();
    for item in &mut block.items {
        if let ImplItem::Fn(ImplItemFn { block, .. }) = item {
            expander.expand(block);
        }
    }

    let gen = Gen {
        attrs: block.attrs.drain(..).collect(),
        vis: attr.vis,
        ty: *block.self_ty.clone(),
        generics: block.generics.clone(),
        fields: expander.fields,
        constructor: attr.constructor,
    };

    TokenStream::from(quote!(
        #gen
        #block
    ))
}
