use panic_to_result_macro::panic_to_result;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
}

#[panic_to_result]
fn create_person(name: String, age: u8) -> Person {
    if age < 18 {
        panic!("Age must be 18 or older");
    }
    Person { name, age }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let actual = create_person("Alice".to_string(), 22).unwrap();

        assert_eq!(actual.name, "Alice".to_string());
        assert_eq!(actual.age, 22);
    }

    #[test]
    fn should_panic_on_invalid_age() {
        let actual = create_person("Alice".to_string(), 17);

        assert_eq!(
            actual.expect_err("this should be an error"),
            "Age must be 18 or older"
        );
    }
}
