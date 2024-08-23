use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

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

#[proc_macro_attribute]
pub fn public(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => unimplemented!("only works for structs with named fields"),
    };

    let builder_fields = fields.iter().map(|f| {
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

struct StructField {
    name: Ident,
    ty: syn::Type,
}
