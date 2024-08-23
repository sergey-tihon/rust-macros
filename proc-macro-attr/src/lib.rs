use proc_macro::TokenStream;
use quote::quote;
use syn::Data::Struct;
use syn::Fields::{Named, Unnamed};
use syn::{parse_macro_input, DataStruct, DeriveInput, FieldsNamed, FieldsUnnamed};

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let builder_fields = named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! { pub #name: #ty }
            });

            let public_version = quote! {
                pub struct #name {
                    #(#builder_fields),*
                }
            };

            public_version.into()
        }
        Struct(DataStruct {
            fields: Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => {
            let fields = unnamed.iter().map(|f| {
                let ty = &f.ty;
                quote! { #ty }
            });

            let public_version = quote! {
                pub struct #name(#(#fields),*);
            };

            public_version.into()
        }
        _ => unimplemented!("only works for structs with named fields"),
    }
}
