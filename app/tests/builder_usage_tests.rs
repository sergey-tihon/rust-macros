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

// #[should_panic]
// #[test]
// fn should_panic_when_field_is_missing() {
//     #[derive(Builder)]
//     struct Gleichnir {
//         _roots_of: String,
//     }
//
//     Gleichnir::builder().build();
// }

#[test]
fn should_generate_builder_for_struct_with_one_renamed_property() {
    #[derive(Builder)]
    struct Gleichnis {
        #[rename("tops_of")]
        root_of: String,
    }

    let gleipnir = Gleichnis::builder()
        .tops_of("mountains".to_string())
        .build();

    assert_eq!(gleipnir.root_of, "mountains".to_string());
}

#[test]
fn should_generate_builder_for_struct_with_two_props_one_custom_name() {
    #[derive(Builder)]
    struct Gleichnis {
        #[rename("tops_of")]
        root_of: String,
        breath_of_a_fish: u8,
    }

    let gleipnir = Gleichnis::builder()
        .tops_of("mountains".to_string())
        .breath_of_a_fish(1)
        .build();

    assert_eq!(gleipnir.root_of, "mountains".to_string());
    assert_eq!(gleipnir.breath_of_a_fish, 1);
}

#[test]
fn should_generate_builder_for_struct_with_one_renamed_prop() {
    #[derive(Builder)]
    struct Gleichnis {
        #[rename = "tops_of"]
        root_of: String,
    }

    let gleipnir = Gleichnis::builder()
        .tops_of("mountains".to_string())
        .build();

    assert_eq!(gleipnir.root_of, "mountains".to_string());
}

#[test]
fn should_use_defaults_when_attribute_is_present() {
    #[derive(Builder)]
    #[builder_defaults]
    struct ExampleStruct {
        string_value: String,
        int_value: i32,
    }

    let example = ExampleStruct::builder().build();

    assert_eq!(example.string_value, String::default());
    assert_eq!(example.int_value, Default::default());
}

#[test]
fn should_work_with_correct_order() {
    #[derive(Builder)]
    struct Gleichnis {
        roots_of: String,
        breath_of_a_fish: u8,
        anything_else: bool,
    }

    let gleipnir = Gleichnis::builder()
        .roots_of("mountains".to_string())
        .breath_of_a_fish(1)
        .anything_else(true)
        .build();

    assert_eq!(gleipnir.roots_of, "mountains".to_string());
}
