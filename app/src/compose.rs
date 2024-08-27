use proc_macro::compose;

fn add_one(n: i32) -> i32 {
    n + 1
}

fn stringify(n: i32) -> String {
    n.to_string()
}

pub fn main() {
    let composed = compose!(add_one.add_one.stringify);

    println!("conposed(1)={:?}", composed(1));
}
