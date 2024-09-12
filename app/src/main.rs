use proc_macro_derive::{Hello, UpperName};

mod compose;
mod macro_rules;
mod panic_to_result;
mod private;
mod public;

// Chapter
#[derive(Hello)]
struct DeriveStruct {}

#[derive(Hello)]
enum Pet {
    Dog,
}

#[derive(UpperName)]
struct UpperStruct {}

fn main() {
    let ds = DeriveStruct {};
    ds.hello_world();

    let pet = Pet::Dog;
    pet.hello_world();

    let upp = UpperStruct {};
    upp.uppercase();

    compose::main();
}
