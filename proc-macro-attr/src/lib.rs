use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data::Struct;
use syn::Field;
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
            let fields = named.iter().map(|f| {
                let name = &f.ident;
                let ty = &f.ty;
                quote! { pub #name: #ty }
            });

            let public_version = quote! {
                pub struct #name {
                    #(#fields),*
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

use proc_macro2::Ident;

struct StructFields {
    name: Option<Ident>,
    ty: Ident,
}

impl Parse for StructFields {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let first = input.cursor().ident().unwrap();
        let res = if first.0.to_string().contains("pub") {
            let second = first.1.ident().unwrap();
            let third = second.1.punct().unwrap().1.ident().unwrap();
            Ok(StructFields {
                name: Some(second.0),
                ty: third.0,
            })
        } else if let Some(colon) = first.1.punct() {
            let second = colon.1.ident().unwrap();
            Ok(StructFields {
                name: Some(first.0),
                ty: second.0,
            })
        } else {
            eprintln!("{:#?}", first.0);
            Ok(StructFields {
                name: None,
                ty: first.0,
            })
        };

        let _: Result<proc_macro2::TokenStream, _> = input.parse();
        res
    }
}

impl ToTokens for StructFields {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ty = &self.ty;
        match &self.name {
            Some(name) => quote! { pub #name: #ty }.to_tokens(tokens),
            None => quote! { #ty }.to_tokens(tokens),
        }
    }
}

fn create_pub_struct(name: Ident, input: &Punctuated<Field, Comma>, is_named: bool) -> TokenStream {
    let fields = input
        .iter()
        .map(|f| syn::parse2::<StructFields>(f.to_token_stream()).unwrap());

    if is_named {
        let public_version = quote! {
            pub struct #name {
                #(#fields),*
            }
        };
        return public_version.into();
    }

    let public_version = quote! {
        pub struct #name(#(#fields),*);
    };
    public_version.into()
}

#[proc_macro_attribute]
pub fn public2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    //eprintln!("{:#?}", &ast);
    let name = ast.ident;

    match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => create_pub_struct(name, named, true),
        Struct(DataStruct {
            fields: Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => create_pub_struct(name, unnamed, false),
        _ => unimplemented!("only works for structs with named fields"),
    }
}
