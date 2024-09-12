mod compose;
mod private;

use proc_macro::TokenStream;

#[proc_macro]
pub fn private(item: TokenStream) -> TokenStream {
    private::main(item)
}

#[proc_macro]
pub fn compose(item: TokenStream) -> TokenStream {
    compose::main(item)
}
