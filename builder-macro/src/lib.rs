use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

#[proc_macro_error]
#[proc_macro_derive(Builder)]
pub fn builder(item: TokenStream) -> TokenStream {
    builder_code::create_builder(item.into()).into()
}
