mod fields;
mod util;

use crate::fields::*;
use proc_macro2::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{Attribute, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

const DEFAULTS_ATTRIBUTE_NAME: &str = "builder_defaults";

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = &ast.ident;

    let fields = match ast.data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => {
            emit_error!(
                ast,
                format!(
                    "only structs with named fields are supported. Struct: {}",
                    quote!(#name)
                )
            );
            return quote! {};
        }
    };

    let use_defaults = use_defaults(&ast.attrs);
    let builder = builder_definition(name, fields);
    let builder_methods_for_struct = builder_impl_for_struct(name, fields);
    let marker_and_structs = marker_trait_and_structs(name, fields);
    let builder_methods = builder_methods(name, fields, use_defaults);

    let default_assertions = if use_defaults {
        optional_default_asserts(fields)
    } else {
        vec![]
    };

    quote! {
        #builder
        #builder_methods_for_struct
        #marker_and_structs
        #builder_methods
        #(#default_assertions)*
    }
}

fn use_defaults(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|a| a.path().is_ident(DEFAULTS_ATTRIBUTE_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_struct_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };

        let actual = create_builder(input);

        assert!(actual.to_string().contains("StructWithNoFieldsBuilder"));
    }

    #[test]
    fn builder_struct_with_expected_methods_should_be_present_in_output() {
        let input = quote! {
            struct StructWithNoFields {}
        };
        let expected = quote! {
            pub struct StructWithNoFieldsBuilder <T: MarkerTraitForBuilder > {
                marker : core::marker::PhantomData <T>,
            }
            impl StructWithNoFields {
                pub fn builder () -> StructWithNoFieldsBuilder<FinalBuilder> {
                    StructWithNoFieldsBuilder {
                        marker: Default::default(),
                    }
                }
            }
            pub trait MarkerTraitForBuilder { }
            pub struct FinalBuilder { }
            impl MarkerTraitForBuilder for FinalBuilder { }
            impl StructWithNoFieldsBuilder<FinalBuilder> {
                pub fn build (self) -> StructWithNoFields {
                    StructWithNoFields { }
                }
            }
        };

        let actual = create_builder(input);

        assert_eq!(actual.to_string(), expected.to_string());
    }

    #[ignore = "not compatible with full macro"]
    #[test]
    fn assert_with_parsing() {
        let input = quote! {
            struct StructWithNoFields {}
        };
        let actual = create_builder(input);

        let derived: DeriveInput = syn::parse2(actual).unwrap();
        let name = derived.ident;
        assert_eq!(name.to_string(), "StructWithNoFieldsBuilder");
    }

    #[test]
    fn builder_struct_with_fields_should_be_present_in_output() {
        let input = quote! {
            struct NumStruct {
                num: u8,
            }
        };
        let expected = quote! {
            pub struct NumStructBuilder <T: MarkerTraitForBuilder> {
                marker: core::marker::PhantomData<T>,
                num : Option <u8>,
            }
            impl NumStruct {
                pub fn builder () -> NumStructBuilder<__NumOfNumStructBuilder> {
                    NumStructBuilder {
                        marker: Default::default(),
                        num:  None,
                    }
                }
            }
            pub trait MarkerTraitForBuilder { }
            pub struct __NumOfNumStructBuilder { }
            impl MarkerTraitForBuilder for __NumOfNumStructBuilder { }
            pub struct FinalBuilder { }
            impl MarkerTraitForBuilder for FinalBuilder { }

            impl NumStructBuilder <__NumOfNumStructBuilder> {
                pub fn num (mut self , input : u8) -> NumStructBuilder<FinalBuilder> {
                    self.num = Some(input) ;
                    NumStructBuilder {
                        marker: Default::default () ,
                        num: self.num,
                    }
                }
            }
            impl NumStructBuilder<FinalBuilder> {
                pub fn build (self) -> NumStruct {
                    NumStruct {
                        num : self.num.expect(concat!("field not set: " , "num")),
                    }
                }
            }
        };

        let actual = create_builder(input);

        assert_eq!(actual.to_string(), expected.to_string());
    }
}
