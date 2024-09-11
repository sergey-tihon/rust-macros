pub fn main() {}

#[cfg(test)]
mod tests {
    use builder_macro::Builder;

    #[test]
    fn should_generate_builder_for_struct_with_no_properties() {
        #[derive(Builder)]
        struct ExampleStructNoFields {}

        let _: ExampleStructNoFields = ExampleStructNoFields::builder().build();
    }

    #[test]
    fn should_generate_builder_for_struct_with_single_property() {
        #[derive(Builder)]
        struct Gleipnir {
            root_of: String,
        }

        let glr = Gleipnir::builder().root_of("mountain".to_string()).build();

        assert_eq!(glr.root_of, "mountain".to_string());
    }
}
