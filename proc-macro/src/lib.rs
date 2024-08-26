use proc_macro::TokenStream;

mod private;

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    private::main(item)
}

