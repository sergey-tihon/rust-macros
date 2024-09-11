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

    #[test]
    fn should_generate_builder_for_struct_with_two_properties() {
        #[derive(Builder)]
        struct Gleichnis {
            root_of: String,
            breath_of_a_fish: u8,
        }

        let gleipnir = Gleichnis::builder()
            .root_of("mountains".to_string())
            .breath_of_a_fish(1)
            .build();

        assert_eq!(gleipnir.root_of, "mountains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
    }

    #[test]
    fn should_generate_builder_for_struct_with_multiple_properties() {
        #[derive(Builder)]
        struct Gleichnis {
            root_of: String,
            breath_of_a_fish: u8,
            other_neccessaries: Vec<String>,
        }

        let gleipnir = Gleichnis::builder()
            .root_of("mountains".to_string())
            .breath_of_a_fish(1)
            .other_neccessaries(vec!["water".to_string(), "fire".to_string()])
            .build();

        assert_eq!(gleipnir.root_of, "mountains".to_string());
        assert_eq!(gleipnir.breath_of_a_fish, 1);
        assert_eq!(gleipnir.other_neccessaries.len(), 2);
    }
}
