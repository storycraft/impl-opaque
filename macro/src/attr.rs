use std::ops::{Deref, DerefMut};

use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    token::Comma,
    Pat, PatIdent, Token, Type, Visibility,
};

pub struct Attr {
    pub vis: Visibility,
    pub constructor: Constructor,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = if input.peek(Token![as]) {
            let _ = input.parse::<Token![as]>()?;

            let vis = Parse::parse(input)?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }

            vis
        } else {
            Visibility::Inherited
        };

        let constructor = Parse::parse(input)?;

        Ok(Self { vis, constructor })
    }
}

#[repr(transparent)]
pub struct Constructor(Punctuated<ConstructorArgs, Comma>);

impl Parse for Constructor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(input)?))
    }
}

impl Deref for Constructor {
    type Target = Punctuated<ConstructorArgs, Comma>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Constructor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for Constructor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        (**self).to_tokens(tokens)
    }
}

pub struct ConstructorArgs {
    pub vis: Option<Visibility>,
    pub pat: PatIdent,
    pub ty: Type,
}

impl Parse for ConstructorArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = if input.peek(Token![pub]) {
            Some(Parse::parse(input)?)
        } else {
            None
        };

        let pat = match Pat::parse_single(input) {
            Ok(Pat::Ident(ident)) => ident,
            Ok(_) => return Err(syn::Error::new(input.span(), "expected ident pattern")),
            Err(err) => return Err(err),
        };

        input.parse::<Token![:]>()?;
        let ty = Parse::parse(input)?;

        Ok(Self { vis, pat, ty })
    }
}

impl ToTokens for ConstructorArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.pat.to_tokens(tokens);

        Token![:](tokens.span()).to_tokens(tokens);

        self.ty.to_tokens(tokens);
    }
}
