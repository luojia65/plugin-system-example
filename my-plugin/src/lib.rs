use my_interface::SayHelloService;

#[no_mangle]
pub extern "Rust" fn new_service() -> Box<dyn SayHelloService> {
    Box::new(PluginSayHello::new())
}

pub struct PluginSayHello {
    id: String,
}

impl PluginSayHello {
    fn new() -> PluginSayHello {
        let id = format!("{:08x}", rand::random::<u32>());
        println!("[{}] Created instance!", id);
        PluginSayHello { id }
    }
}

impl SayHelloService for PluginSayHello {
    fn say_hello(&self) {
        println!("[{}] Hello from plugin!", self.id);
    }
}

impl Drop for PluginSayHello {
    fn drop(&mut self) {
        println!("[{}] Destroyed instance!", self.id);
    }
}
