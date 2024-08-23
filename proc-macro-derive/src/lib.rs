use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hello)]
pub fn hello(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let add_hello_world = quote! {
        impl #name {
            fn hello_world(&self) {
                println!("Hello, World")
            }
        }
    };

    add_hello_world.into()
}

#[proc_macro_derive(UpperName)]
pub fn uppercase(item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;
    let upper_name = name.to_string().to_uppercase();

    let add_upper_case = quote! {
        impl #name {
            fn uppercase(&self) {
                println!("{}", #upper_name);
            }
        }
    };
    add_upper_case.into()
}
