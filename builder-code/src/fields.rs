use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, Field, Ident, Type};

pub fn original_struct_setters(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = TokenStream> + '_ {
    fields.iter().map(|f| {
        let (field_name, field_type) = get_name_and_type(f);
        let field_name_as_string = field_name.as_ref().unwrap().to_string();
        let error = quote!(expect(&format!("field {} not set", #field_name_as_string)));

        let handle_type = if matches_type(field_type, "String") {
            quote! { as_ref().#error.to_string() }
        } else {
            quote! { #error }
        };

        quote! {
            #field_name: self.#field_name.#handle_type
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

fn matches_type(ty: &Type, type_name: &str) -> bool {
    if let Type::Path(ref p) = ty {
        let first_match = p.path.segments[0].ident.to_string();
        return first_match == *type_name;
    }
    false
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
