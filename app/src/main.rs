mod macro_rules;

use proc_macro_attr::public;
use proc_macro_derive::{Hello, UpperName};

#[derive(Hello)]
struct DeriveStruct {}

#[derive(Hello)]
enum Pet {
    Dog,
}

#[derive(UpperName)]
struct UpperStruct {}

#[public]
struct Example {
    first: String,
    pub second: u32,
}

fn main() {
    let ds = DeriveStruct {};
    ds.hello_world();

    let pet = Pet::Dog;
    pet.hello_world();

    let upp = UpperStruct {};
    upp.uppercase();
}
