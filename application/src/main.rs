// unfortunately you need to use include! here
// to get it to work, because you can't
// implement a trait on an external crate
include!("../../definitions/src/lib.rs");

use anyhow::Result;

use libloading::Library;

// IExample is implemented here, instead of in the definitions
// commenting this line out will make the application not compile
impl IExample for Example {
    fn print(&self) {
        println!("hello {}! you are {} years old!", self.m_name, self.m_age);
    }
}

pub fn main() -> Result<()> {
    let lib = unsafe { Library::new("plugin_example")? };
    let plugin = unsafe { lib.get::<&Plugin>(b"dll_info")? };

    let mut interface = Interface::default();

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

    let mut user = Example {
        m_name: "john doe".to_string(),
        m_age: 42,
    };
    for cb in interface.get_callbacks() {
        cb(&mut user);
    }
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
