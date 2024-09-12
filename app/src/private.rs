use proc_macro::private;

private! {
    #[allow(dead_code)]
    struct Example {
        string_value: String,
        number_value: u32
    }
}
