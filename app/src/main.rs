use libloading::{Library, Symbol};
mod apis_impl;

pub fn load_and_run_plugin(plugin_name: &'static str) {
    unsafe {
        let lib = Library::new(format!("./target/debug/lib{}.dylib", plugin_name))
            .expect("Failed to load plugin");

        let init_function: Symbol<fn(*const apis::ApisData)> =
            lib.get(b"init_apis").expect("Failed to get init function");
        init_function(&apis_impl::APIS);

        let main_function: Symbol<fn()> = lib.get(b"main").expect("Failed to get main function");
        main_function();
    }
}

fn main() {
    load_and_run_plugin("plugin1");
    load_and_run_plugin("plugin2");
    //unsafe {
    //    // Loading plguin 1
    //
    //
    //    //let lib = Library::new("./target/debug/libplugin1.dylib").expect("Failed to load plugin");
    //    //let plugin_1_func: Symbol<fn(*const apis::ApisData)> =
    //    //    lib.get(b"init_apis").expect("Failed to get function");
    //    //
    //    //// Loading plguin 2
    //    //let lib = Library::new("./target/debug/libplugin2.dylib").expect("Failed to load plugin");
    //    //let plugin_2_func: Symbol<fn(*const apis::ApisData)> =
    //    //    lib.get(b"init_apis").expect("Failed to get function");
    //    //
    //    //// println!("apis in app {:?}", apis_impl::APIS.val);
    //    //plugin_1_func(&apis_impl::APIS);
    //    //plugin_2_func(&apis_impl::APIS);
    //}
}
