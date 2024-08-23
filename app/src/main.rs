// macro_rules! hello_world {
//     ($something:ident) => {
//         impl $something {
//             fn hello_world(&self) {
//                 println!("Hello world")
//             }
//        }
//     };
// }

use proc_macro::{public, Hello, UpperName};

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
