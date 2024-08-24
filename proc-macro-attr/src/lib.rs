use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::Data::Struct;
use syn::Fields::{Named, Unnamed};
use syn::Visibility;
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

use proc_macro2::Ident;

struct StructFields {
    name: Ident,
    ty: Ident,
}

impl Parse for StructFields {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let _vis = input.parse::<Visibility>()?;
        let list = Punctuated::<Ident, Colon>::parse_terminated(input).unwrap();

        Ok(StructFields {
            name: list.first().unwrap().clone(),
            ty: list.last().unwrap().clone(),
        })
    }
}

impl ToTokens for StructFields {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = &self.name;
        let ty = &self.ty;
        quote! { pub #name: #ty }.to_tokens(tokens);
    }
}

#[proc_macro_attribute]
pub fn public2(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let name = ast.ident;

    match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => {
            let builder_fields = named
                .iter()
                .map(|f| syn::parse2::<StructFields>(f.to_token_stream()).unwrap());

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
