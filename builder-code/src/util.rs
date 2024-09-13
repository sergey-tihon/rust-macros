use proc_macro2::Ident;
use quote::format_ident;

pub fn create_builder_ident(name: &Ident) -> Ident {
    format_ident!("{}Builder", name)
}

pub fn create_field_struct_name(builder: &Ident, field: &Ident) -> Ident {
    let prefix = snake_to_camel(field);
    format_ident!("__{}Of{}", prefix, builder)
}

pub fn snake_to_camel(field_name: &Ident) -> Ident {
    let field_name = field_name.to_string();
    let mut camel_name = String::new();
    let mut capitalize = true;
    for c in field_name.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            camel_name.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            camel_name.push(c);
        }
    }
    format_ident!("{}", camel_name)
}
