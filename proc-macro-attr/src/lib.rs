use proc_macro::TokenStream;

mod public;
mod public2;

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    public::main(_attr, item)
}

#[proc_macro_attribute]
pub fn public2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    public2::main(_attr, item)
}
