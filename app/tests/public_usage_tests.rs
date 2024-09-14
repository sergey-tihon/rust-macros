mod sub_module {
    use proc_macro_attr::public2;

    #[public2(exclude(fourth, third))]
    struct ExampleEx {
        first: String,
        pub second: u32,
        third: String,
        fourth: u32,
    }

    impl ExampleEx {
        pub fn new() -> Self {
            ExampleEx {
                first: Default::default(),
                second: Default::default(),
                third: Default::default(),
                fourth: Default::default(),
            }
        }
    }
}

#[test]
fn should_exclude_some_fields() {
    let e = sub_module::ExampleEx::new();
    assert_eq!(e.first, String::new());
    assert_eq!(e.second, Default::default());
    //assert_eq!(e.third, String::new());
    //assert_eq!(e.fourth, Default::default());
}
