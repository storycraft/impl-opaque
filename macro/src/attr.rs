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
    pub constness: Option<Token![const]>,
    pub new_args: NewArgs,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (vis, constness) = if input.peek(Token![as]) {
            let _ = input.parse::<Token![as]>()?;

            let vis = Parse::parse(input)?;
            let constness = Parse::parse(input)?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }

            (vis, constness)
        } else {
            (Visibility::Inherited, None)
        };

        let constructor = Parse::parse(input)?;

        Ok(Self {
            vis,
            constness,
            new_args: constructor,
        })
    }
}

#[repr(transparent)]
pub struct NewArgs(Punctuated<Argument, Comma>);

impl Parse for NewArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(input)?))
    }
}

impl Deref for NewArgs {
    type Target = Punctuated<Argument, Comma>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NewArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for NewArgs {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        (**self).to_tokens(tokens)
    }
}

pub struct Argument {
    pub vis: Option<Visibility>,
    pub pat: PatIdent,
    pub ty: Type,
}

impl Parse for Argument {
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

impl ToTokens for Argument {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.pat.to_tokens(tokens);

        Token![:](tokens.span()).to_tokens(tokens);

        self.ty.to_tokens(tokens);
    }
}
