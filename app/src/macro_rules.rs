macro_rules! hello_world {
    ($something:ident) => {
        impl $something {
            fn hello_world(&self) {
                println!("Hello world")
            }
        }
    };
}

struct World;

hello_world!(World);

impl World {
    #[allow(dead_code)]
    pub fn print(&self) {
        self.hello_world();
    }
}
