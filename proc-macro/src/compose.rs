use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::Token;

struct ComposeInput {
    expressions: Punctuated<Ident, Token!(.)>,
}

impl Parse for ComposeInput {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let expressions = Punctuated::<Ident, Token!(.)>::parse_terminated(input).unwrap();
        Ok(Self { expressions })
    }
}

impl ToTokens for ComposeInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut result = None;
        let mut as_idents: Vec<&Ident> = self.expressions.iter().collect();
        let last_ident = as_idents.pop().unwrap();

        as_idents.iter().rev().for_each(|ident| {
            if let Some(current) = &result {
                result = Some(quote! { compose_two(#ident, #current) });
            } else {
                result = Some(quote! { compose_two(#ident, #last_ident) });
            }
        });
        result.to_tokens(tokens);
    }
}

pub fn main(item: TokenStream) -> TokenStream {
    eprintln!("{:#?}", item);
    let ci: ComposeInput = syn::parse_macro_input!(item);

    quote! {
        {
            fn compose_two<FIRST, SECOND, THIRD, F, G>(first: F, second: G) -> impl Fn(FIRST) -> THIRD
            where
                F: Fn(FIRST) -> SECOND,
                G: Fn(SECOND) -> THIRD,
            {
                move |x| second(first(x))
            }
            #ci
        }
    }
    .into()
}
