#![doc = include_str!("../README.md")]

use std::{collections::HashMap, fs};

use input::ConfigInput;
use proc_macro::TokenStream;

mod input;
#[cfg(feature = "functional")]
mod output;
#[cfg(feature = "struct")]
mod output_struct;

fn find_yaml_values(input: ConfigInput) -> Result<HashMap<String, String>, syn::Error> {
    let file_name = input
        .path
        .unwrap_or_else(|| "./configuration/config.yaml".to_string());
    let file = fs::File::open(&file_name).map_err(|err| {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("could not read config with path {}: {}", file_name, err),
        )
    })?;

    serde_yaml::from_reader(file)
        .map_err(|err| syn::Error::new(proc_macro2::Span::call_site(), err.to_string()))
}

/// This function-like macro will generate a struct called `Config`
/// which contains a ‘HashMap<String,String>’ with all
/// the yaml config properties.
#[cfg(feature = "functional")]
#[proc_macro]
pub fn config(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as ConfigInput);
    match find_yaml_values(input) {
        Ok(values) => output::generate_config_struct(values).into(),
        Err(err) => err.to_compile_error().into(),
    }
}

/// This macro allows manipulation of an existing struct
/// to serve as a 'config' struct.
/// It will replace any existing fields with those present
/// in the configuration.
///
/// ```rust
/// use config_macro::config_struct;
///
/// #[config_struct(path  = "./configuration/config.yaml")]
/// struct Example {}
///
/// // Example now has a new method
/// let e = Example::new();
///
/// // e now contains a 'user' field that we can access
/// println!("{}", e.user);
/// ```
///
#[cfg(any(feature = "struct", doc))]
#[proc_macro_attribute]
pub fn config_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(attr as ConfigInput);
    let ast = syn::parse_macro_input!(item as syn::DeriveInput);

    match find_yaml_values(input) {
        Ok(values) => output_struct::generate_annotation_struct(ast, values).into(),
        Err(err) => err.to_compile_error().into(),
    }
}
