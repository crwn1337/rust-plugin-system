// unfortunately you need to use include! here
// to get it to work, because you can't
// implement a trait on an external crate (no example yet)
include!("../../definitions/src/lib.rs");

use std::error::Error;

use libloading::Library;

pub fn main() -> Result<(), Box<dyn Error>> {
    let lib = unsafe { Library::new("plugin_example").map_err(|_| "couldnt find plugin")? };
    let plugin_info = unsafe {
        lib.get::<fn() -> &'static Plugin>(b"dll_info")
            .map_err(|_| "couldnt find dll_init")?
    };

    let mut interface = Interface::new();

    let plugin = plugin_info();
    println!("plugin name: {}", plugin.m_name);
    println!("authors: {:?}", plugin.m_authors);
    println!();

    println!("running dll's init");
    match plugin.m_init {
        Some(init) => init(&mut interface),
        None => println!("no init function?"),
    }

    println!("callback count: {}", interface.get_callbacks().len());
    println!();

    println!("hello (should be true) : {}", interface.get_callbacks()[0]("hello".to_string()));
    println!("world (should be false): {}", interface.get_callbacks()[0]("world".to_string()));
    println!();

    println!("running dll's shutdown");
    match plugin.m_shutdown {
        Some(shutdown) => shutdown(&mut interface),
        None => println!("no shutdown function?"),
    }
    println!("callback count: {}", interface.get_callbacks().len());
    println!();

    println!("finished executing");
    Ok(())
}
