use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Comma,
    PatType, Token, Visibility,
};

pub struct Attr {
    pub vis: Visibility,
    pub constructor: Constructor,
}

impl Parse for Attr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vis = if input.peek(Token![pub]) {
            let vis = Visibility::parse(input)?;

            if !input.is_empty() {
                input.parse::<Token![,]>()?;
            }

            vis
        } else {
            Visibility::Inherited
        };

        let constructor = Constructor::parse(input)?;

        Ok(Self { vis, constructor })
    }
}

#[repr(transparent)]
pub struct Constructor(Punctuated<PatType, Comma>);

impl Parse for Constructor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(Punctuated::parse_terminated(input)?))
    }
}

impl ToTokens for Constructor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }
}
