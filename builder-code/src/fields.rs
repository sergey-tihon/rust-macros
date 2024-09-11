use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field, Ident, Type};

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, _) = get_name_and_type(f);
        let field_name_as_string = field_name.as_ref().unwrap().to_string();
        quote! {
            #field_name: self.#field_name.as_ref()
                .expect(
                    &format!("field {} not set", #field_name_as_string)
                )
                .to_string()
        }
    })
}

pub fn builder_methods(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! {
            pub fn #field_name(&mut self, input: #field_type) -> &mut Self {
                self.#field_name = Some(input);
                self
            }
        }
    })
}

pub fn builder_init_values(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, _) = get_name_and_type(f);
        quote! {
            #field_name: None
        }
    })
}

pub fn builder_field_definitions(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! {
            #field_name: Option<#field_type>
        }
    })
}

fn get_name_and_type(f: &Field) -> (&Option<Ident>, &Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
}
