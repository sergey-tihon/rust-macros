mod fields;

use crate::fields::*;
use proc_macro2::TokenStream;
use proc_macro_error::emit_error;
use quote::{format_ident, quote};
use syn::{Attribute, Data::Struct, DataStruct, DeriveInput, Fields::Named, FieldsNamed};

const DEFAULTS_ATTRIBUTE_NAME: &str = "builder_defaults";

pub fn create_builder(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = &ast.ident;
    let builder = format_ident!("{}Builder", name);
    let use_defaults = use_defaults(&ast.attrs);

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

    let builder_fields = builder_field_definitions(fields);
    let builder_inits = builder_init_values(fields);
    let builder_methods = builder_methods(fields);
    let set_fields = original_struct_setters(fields, use_defaults);

    quote! {
        struct #builder {
            #(#builder_fields,)*
        }

        impl #builder {
            #(#builder_methods)*

            pub fn build(self) -> #name {
                #name {
                    #(#set_fields,)*
                }
            }
        }

        impl #name {
            pub fn builder() -> #builder {
                #builder {
                    #(#builder_inits,)*
                }
            }
        }
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
            struct StructWithNoFieldsBuilder {}

            impl StructWithNoFieldsBuilder {
                pub fn build(self) -> StructWithNoFields {
                    StructWithNoFields {}
                }
            }

            impl StructWithNoFields {
                pub fn builder() -> StructWithNoFieldsBuilder {
                    StructWithNoFieldsBuilder {}
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
            struct NumStructBuilder {
                num: Option<u8>,
            }

            impl NumStructBuilder {
                pub fn num (mut self , input : u8) -> Self {
                    self.num = Some(input) ;
                    self
                }
                pub fn build (self) -> NumStruct {
                    NumStruct {
                        num : self.num.expect(concat!("field not set: ","num")),
                    }
                }
            }

            impl NumStruct {
                pub fn builder () -> NumStructBuilder {
                    NumStructBuilder { num : None , }
                }
            }
        };

        let actual = create_builder(input);

        assert_eq!(actual.to_string(), expected.to_string());
    }
}
