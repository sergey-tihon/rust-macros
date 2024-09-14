use proc_macro::TokenStream;
use proc_macro2::Span;

use quote::quote;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::Data::Struct;
use syn::Field;
use syn::Fields::{Named, Unnamed};
use syn::{
    parse_macro_input, DataStruct, DeriveInput, FieldsNamed, FieldsUnnamed, Ident, MetaList, Token,
};

#[derive(Debug)]
struct StructFields {
    name: Option<Ident>,
    ty: Ident,
    vis: Option<Ident>,
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
                vis: Some(first.0),
            })
        } else if let Some(colon) = first.1.punct() {
            let second = colon.1.ident().unwrap();
            Ok(StructFields {
                name: Some(first.0),
                ty: second.0,
                vis: None,
            })
        } else {
            // eprintln!("{:#?}", first.0);
            Ok(StructFields {
                name: None,
                ty: first.0,
                vis: None,
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
            Some(name) => {
                if let Some(vis) = &self.vis {
                    quote! { #vis #name: #ty }.to_tokens(tokens)
                } else {
                    quote! { #name: #ty }.to_tokens(tokens)
                }
            }
            None => quote! { #ty }.to_tokens(tokens),
        }
    }
}

const EXCLUDE_ATTRIBUTE_NAME: &str = "exclude";

struct ExcludedFields {
    fields: Vec<String>,
}

impl ExcludedFields {
    fn marches_ident(&self, ident: &Option<Ident>) -> bool {
        ident
            .as_ref()
            .map(|n| n.to_string())
            .map(|n| self.fields.iter().any(|f| *f == n))
            .unwrap_or(false)
    }
}

impl Parse for ExcludedFields {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        match input.parse::<MetaList>() {
            Ok(meta_list) => {
                if meta_list
                    .path
                    .segments
                    .iter()
                    .any(|s| s.ident == EXCLUDE_ATTRIBUTE_NAME)
                {
                    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
                    let identifiers = parser.parse(meta_list.clone().tokens.into()).unwrap();
                    let fields = identifiers.iter().map(|v| v.to_string()).collect();
                    Ok(ExcludedFields { fields })
                } else {
                    Ok(ExcludedFields { fields: vec![] })
                }
            }
            Err(_) => Ok(ExcludedFields { fields: vec![] }),
        }
    }
}

fn create_pub_struct(
    name: Ident,
    input: &Punctuated<Field, Comma>,
    is_named: bool,
    excluded_fields: ExcludedFields,
) -> TokenStream {
    let fields = input
        .iter()
        .map(|f| syn::parse2::<StructFields>(f.to_token_stream()).unwrap())
        .map(|mut f| {
            if !excluded_fields.marches_ident(&f.name) {
                f.vis = Some(Ident::new("pub", Span::call_site()));
            }
            f
        });

    let public_version = if is_named {
        quote! {
            pub struct #name {
                #(#fields),*
            }
        }
    } else {
        quote! {
            pub struct #name(#(#fields),*);
        }
    };

    eprintln!("{}", public_version);

    public_version.into()
}

pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(item as DeriveInput);
    let excluded_fields = parse_macro_input!(attr as ExcludedFields);
    //eprintln!("{:#?}", &ast);
    let name = ast.ident;

    match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => create_pub_struct(name, named, true, excluded_fields),
        Struct(DataStruct {
            fields: Unnamed(FieldsUnnamed { ref unnamed, .. }),
            ..
        }) => create_pub_struct(name, unnamed, false, excluded_fields),
        _ => unimplemented!("only works for structs with named fields"),
    }
}
