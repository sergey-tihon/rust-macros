use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn main(item: TokenStream) -> TokenStream {
    let items_as_stream: quote::__private::TokenStream = item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    quote! {
        #items_as_stream

        impl #name {}
    }
    .into()
}
