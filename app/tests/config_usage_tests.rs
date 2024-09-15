use config_macro::config_struct;

#[test]
fn should_load_config_from_file() {
    #[config_struct(path = "./app/config.yaml")]
    #[derive(Debug)]
    struct ConfigStruct {}

    let config = ConfigStruct::new();
    println!("{config:?}");
}
