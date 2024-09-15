use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;

fn generate_insert(yaml_values: HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values
        .iter()
        .map(|v| {
            let key = v.0;
            let value = v.1;
            quote! {
                map.insert(#key, #value);
            }
        })
        .collect()
}

pub fn generate_config_struct(yaml_values: HashMap<String, String>) -> TokenStream {
    let insert = generate_insert(yaml_values);
    quote! {
        pub struct Config(
            pub std::collections::HashMap<&'static str, &'static str>
        );
        impl Config {
            pub fn new() -> Self {
                let mut map = std::collections::HashMap::new();
                #(#insert)*
                Config(map)
            }
        }
    }
}
