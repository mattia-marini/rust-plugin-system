#[no_mangle]
pub extern "C" fn main(apis_ptr: *const apis::Apis) {
    println!("Hello from plugin 1!");
    let apis: &apis::Apis;
    unsafe {
        apis = &*apis_ptr;
    }

    (apis.api1)();
    (apis.api2)();
}

//#[link(name = "my_plugin")]
//extern "C" {
//    fn add(a: u32, b: u32) -> u32;
//}
