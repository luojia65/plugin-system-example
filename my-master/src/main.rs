use my_interface::SayHelloService;

fn main() {
    let lib = libloading::Library::new("target/debug/my_plugin.dll")
        .expect("load library");
    let new_service: libloading::Symbol<extern "Rust" fn() -> Box<dyn SayHelloService>> = unsafe { lib.get(b"new_service") }
        .expect("load symbol");
    let service = new_service();
    service.say_hello();
    let service = new_service();
    service.say_hello();
}
