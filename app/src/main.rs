use libloading::{Library, Symbol};
mod apis_impl;

//struct Apis {
//    value: i32,
//}

fn main() {
    unsafe {
        // Loading plguin 1
        let lib = Library::new("./target/debug/libplugin1.dylib").expect("Failed to load plugin");
        let plugin_1_func: Symbol<fn(*const apis::Apis)> =
            lib.get(b"main").expect("Failed to get function");

        // Loading plguin 2
        let lib = Library::new("./target/debug/libplugin2.dylib").expect("Failed to load plugin");
        let plugin_2_func: Symbol<fn(*const apis::Apis)> =
            lib.get(b"main").expect("Failed to get function");

        // println!("apis in app {:?}", apis_impl::APIS.val);
        plugin_1_func(&apis_impl::APIS);
        plugin_2_func(&apis_impl::APIS);
    }
}
