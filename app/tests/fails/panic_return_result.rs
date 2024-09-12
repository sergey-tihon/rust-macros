use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

#[panic_to_result]
fn create_person(name: String, age: u8) -> Result<Person, String> {
    if age < 18 {
        panic!("Age must be 18 or older");
    }
    Ok(Person { name, age })
}

fn main() {}
