mod macro_rules;

use proc_macro_attr::{public, public2};
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
struct Example2 {
    first: String,
    pub second: u32,
}

#[public]
struct Example3(u32, String);

#[public2]
struct Example4 {
    first: String,
    pub second: u32,
}

#[public2]
struct Example5(u32, String);

fn main() {
    let ds = DeriveStruct {};
    ds.hello_world();

    let pet = Pet::Dog;
    pet.hello_world();

    let upp = UpperStruct {};
    upp.uppercase();
}
