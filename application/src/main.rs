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
    let plugin = unsafe { lib.get::<fn() -> Plugin>(b"__dll_info")? };
    let mut plugin = plugin();

    let mut interface = Interface::default();

    println!("plugin name: {}", plugin.m_name);
    println!("authors: {:?}", plugin.m_authors);
    println!("counter: {}", interface.m_counter);
    println!();

    println!("running dll's init");
    match plugin.m_init {
        Some(init) => init(&mut interface),
        None => println!("no init function?"),
    }
    println!("counter: {}", interface.m_counter);

    let mut user = Example {
        m_name: "john doe".to_string(),
        m_age: 42,
    };
    plugin.m_state.callback(&mut user);
    println!();

    println!("running dll's shutdown");
    match plugin.m_shutdown {
        Some(shutdown) => shutdown(&mut interface),
        None => println!("no shutdown function?"),
    }
    println!("counter: {}", interface.m_counter);
    println!();

    println!("finished executing");
    Ok(())
}
