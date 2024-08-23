macro_rules! hello_world {
    ($something:ident) => {
        impl $something {
            fn hello_world(&self) {
                println!("Hello world")
            }
        }
    };
}
