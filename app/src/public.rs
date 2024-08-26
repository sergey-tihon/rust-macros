use proc_macro_attr::{public, public2};

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
