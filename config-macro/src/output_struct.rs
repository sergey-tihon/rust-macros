use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::quote;

fn generate_fields(yaml_values: &HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values
        .iter()
        .map(|v| {
            let key = syn::Ident::new(v.0, Span::call_site());
            quote! {
                pub #key: String
            }
        })
        .collect()
}

fn generate_inits(yaml_values: &HashMap<String, String>) -> Vec<TokenStream> {
    yaml_values
        .iter()
        .map(|v| {
            let key = syn::Ident::new(v.0, Span::call_site());
            let value = v.1;
            quote! {
                #key: #value.to_string()
            }
        })
        .collect()
}

pub fn generate_annotation_struct(
    input: syn::DeriveInput,
    yaml_values: HashMap<String, String>,
) -> TokenStream {
    let attibutes = &input.attrs;
    let name = &input.ident;
    let fields = generate_fields(&yaml_values);
    let inits = generate_inits(&yaml_values);
    quote! {
        #(#attibutes)*
        pub struct #name {
            #(#fields),*
        }

        impl #name {
            pub fn new() -> Self {
                Self {
                    #(#inits),*
                }
            }
        }
    }
}
