use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, ExprLit, Field, Ident, Lit,
    LitStr, Meta, MetaNameValue, Type,
};

use crate::util::*;

pub fn builder_methods(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
    use_defaults: bool,
) -> TokenStream {
    let builder_name = create_builder_ident(struct_name);
    let set_fields = original_struct_setters(fields, use_defaults);
    let assignments_for_all_fields = get_assignments_for_fields(fields);
    let mut previous_field = None;

    let build_method = quote! {
        pub fn build(self) -> #struct_name {
            #struct_name {
                #(#set_fields,)*
            }
        }
    };

    let reversed_names_and_types: Vec<&Field> = fields.iter().rev().collect();
    let methods: Vec<TokenStream> = reversed_names_and_types
        .iter()
        .map(|f| {
            let build_in_builder = if use_defaults {
                build_method.clone()
            } else {
                quote! {}
            };

            let tokens = builder_for_field(
                &builder_name,
                &assignments_for_all_fields,
                f,
                previous_field,
                build_in_builder,
            );
            previous_field = Some(f);
            tokens
        })
        .collect();

    quote! {
        #(#methods)*

        impl #builder_name<FinalBuilder> {
            #build_method
        }
    }
}

fn builder_for_field(
    builder_name: &Ident,
    field_assignments: &Vec<TokenStream>,
    current_field: &Field,
    next_field_in_list: Option<&Field>,
    build_method: TokenStream,
) -> TokenStream {
    let (field_name, field_type) = get_name_and_type(current_field);
    let method_name = method_name(current_field);

    let current_field_struct_name =
        create_field_struct_name(builder_name, field_name.as_ref().unwrap());

    let next_field_struct_name = if let Some(next_field) = next_field_in_list {
        let (next_field_name, _) = get_name_and_type(next_field);
        create_field_struct_name(builder_name, next_field_name.as_ref().unwrap())
    } else {
        format_ident!("FinalBuilder")
    };

    quote! {
        impl #builder_name<#current_field_struct_name> {
            pub fn #method_name(mut self, input: #field_type) -> #builder_name<#next_field_struct_name> {
                self.#field_name = Some(input);
                #builder_name {
                    marker: Default::default(),
                    #(#field_assignments,)*
                }
            }
            #build_method
        }
    }
}

fn get_assignments_for_fields(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let (field_name, _) = get_name_and_type(f);

            quote! {
                #field_name: self.#field_name
            }
        })
        .collect()
}

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
    use_defaults: bool,
) -> Vec<TokenStream> {
    fields
        .iter()
        .map(move |f| {
            let (field_name, _) = get_name_and_type(f);
            let field_name_as_string = field_name.as_ref().unwrap().to_string();

            let handle_type = if use_defaults {
                default_fallback()
            } else {
                panic_fallback(field_name_as_string)
            };

            quote! {
                #field_name: self.#field_name.#handle_type
            }
        })
        .collect()
}

fn panic_fallback(field_name_as_string: String) -> TokenStream {
    quote! {
        expect(concat!("field not set: ", #field_name_as_string))
    }
}

fn default_fallback() -> TokenStream {
    quote! {
        unwrap_or_default()
    }
}

pub fn marker_trait_and_structs(
    struct_name: &syn::Ident,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let builder_name = create_builder_ident(struct_name);

    let struct_and_impls = fields.iter().map(|f| {
        let field_name = &f.ident.clone().unwrap();
        let field_struct_name = create_field_struct_name(&builder_name, field_name);
        quote! {
            pub struct #field_struct_name {}
            impl MarkerTraitForBuilder for #field_struct_name {}
        }
    });

    // TODO: add unique name for trait and final builder
    quote! {
         pub trait MarkerTraitForBuilder {}

         #(#struct_and_impls)*

         pub struct FinalBuilder {}
         impl MarkerTraitForBuilder for FinalBuilder {}
    }
}

pub fn builder_impl_for_struct(
    struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    let builder_inits = fields.iter().map(|f| {
        let (field_name, _) = get_name_and_type(f);
        quote! { #field_name: None }
    });
    let builder_name = create_builder_ident(struct_name);
    let generic = if let Some(f) = fields.first() {
        let first_field_name = f.ident.clone().unwrap();
        create_field_struct_name(&builder_name, &first_field_name)
    } else {
        format_ident!("FinalBuilder")
    };

    quote! {
        impl #struct_name {
            pub fn builder() -> #builder_name<#generic> {
                #builder_name {
                    marker: Default::default(),
                    #(#builder_inits,)*
                }
            }
        }
    }
}

pub fn builder_definition(struct_name: &Ident, fields: &Punctuated<Field, Comma>) -> TokenStream {
    let builder_fields = fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        quote! { #field_name: Option<#field_type> }
    });
    let builder_name = create_builder_ident(struct_name);

    quote! {
        pub struct #builder_name<T: MarkerTraitForBuilder> {
            marker: core::marker::PhantomData<T>,
            #(#builder_fields,)*
        }
    }
}

fn get_name_and_type(f: &Field) -> (&Option<Ident>, &Type) {
    let field_name = &f.ident;
    let field_type = &f.ty;
    (field_name, field_type)
}

pub fn optional_default_asserts(fields: &Punctuated<Field, Comma>) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|f| {
            let ty = &f.ty;
            let ty_name = quote!(#ty).to_string();
            let assertion_ident = format_ident!("__{}DefaultAssertion", ty_name);

            quote_spanned! {ty.span() =>
                struct #assertion_ident where #ty: core::default::Default;
            }
        })
        .collect()
}

fn method_name(f: &Field) -> Ident {
    extract_attribure_from_field(f, "rename")
        .map(|a| &a.meta)
        .map(|m| match m {
            Meta::List(nested) => {
                let a: LitStr = nested.parse_args().unwrap();
                Ident::new(&a.value(), a.span())
            }
            Meta::Path(_) => {
                panic!("expected brackets with name of prop")
            }
            Meta::NameValue(meta) => match meta {
                MetaNameValue {
                    value:
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }),
                    ..
                } => Ident::new(&lit_str.value(), lit_str.span()),
                _ => panic!("expected string literal"),
            },
        })
        .unwrap_or_else(|| f.ident.clone().unwrap())
}

fn extract_attribure_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a syn::Attribute> {
    f.attrs.iter().find(|&a| a.path().is_ident(name))
}

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use syn::{
        punctuated::Punctuated, Field, FieldMutability, Ident, Path, PathSegment, Type, TypePath,
        Visibility,
    };

    use super::get_name_and_type;

    #[test]
    fn get_name_and_type_give_back_name() {
        let p = PathSegment {
            ident: Ident::new("String", Span::call_site()),
            arguments: Default::default(),
        };
        let mut pun = Punctuated::new();
        pun.push(p);
        let ty = Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: pun,
            },
        });
        let f = Field {
            attrs: vec![],
            vis: Visibility::Inherited,
            mutability: FieldMutability::None,
            ident: Some(Ident::new("example", Span::call_site())),
            colon_token: None,
            ty,
        };

        let (actual_name, _) = get_name_and_type(&f);

        assert_eq!(
            actual_name.as_ref().unwrap().to_string(),
            "example".to_string()
        )
    }
}
