// unfortunately you need to use include! here
// to get it to work, because you can't
// implement a trait on an external crate
include!("../../definitions/src/lib.rs");

use std::error::Error;

use libloading::Library;

// IExample is implemented here, instead of the definitions
// commenting this line out will make the application not compile
impl IExample for Example {
    fn print(&self) {
        println!("Hello, {}! You are {} years old!", self.m_name, self.m_age);
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let lib = unsafe { Library::new("plugin_example").map_err(|_| "couldnt find plugin")? };
    let plugin_info = unsafe {
        lib.get::<fn() -> &'static Plugin>(b"dll_info")
            .map_err(|_| "couldnt find dll_init")?
    };

    let mut interface = Interface::default();

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

    let user = Example {
        m_name: "John Doe".to_string(),
        m_age: 42,
    };
    interface.get_callbacks()[0](&user);
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
