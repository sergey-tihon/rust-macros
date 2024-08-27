use proc_macro::TokenStream;
use quote::quote;
use syn::__private::{Span, TokenStream2};
use syn::{
    parse_macro_input, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed, Ident,
};

fn generate_methods(ast: &DeriveInput) -> Vec<TokenStream2> {
    let named_fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Only structs are supported"),
    };

    named_fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_type = &field.ty;
            let method_name = Ident::new(&format!("get_{}", field_name), Span::call_site());

            quote! {
                pub fn #method_name(&self) -> &#field_type {
                    &self.#field_name
                }
            }
        })
        .collect()
}

pub fn main(item: TokenStream) -> TokenStream {
    let items_as_stream: quote::__private::TokenStream = item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    let name = &ast.ident;
    let methods = generate_methods(&ast);

    quote! {
        #items_as_stream

        impl #name {
            #(#methods)*
        }
    }
    .into()
}
