use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Expr, ExprLit, Field, Ident, Lit,
    LitStr, Meta, MetaNameValue, Type,
};

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

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
    use_defaults: bool,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(move |f| {
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

pub fn builder_methods(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        let attr = extract_attribure_from_field(f, "rename")
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
            });
        let method_name = attr.unwrap_or_else(|| field_name.clone().unwrap());

        quote! {
            pub fn #method_name(mut self, input: #field_type) -> Self {
                self.#field_name = Some(input);
                self
            }
        }
    })
}

fn extract_attribure_from_field<'a>(f: &'a Field, name: &'a str) -> Option<&'a syn::Attribute> {
    f.attrs.iter().find(|&a| a.path().is_ident(name))
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
