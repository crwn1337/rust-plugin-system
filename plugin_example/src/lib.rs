include!("../../definitions/src/lib.rs");

pub struct State {
    pub m_test: u32,
}

impl IPlugin for State {
    fn callback(&mut self, example: &mut dyn IExample) -> bool {
        example.m_age = 23;
        println!("printing from the plugin: {}", self.m_test);
        example.print();
        true
    }
}

pub fn init(interface: &mut dyn IInterface) {
    // example of what you might want to do with an interface
    interface.inc_counter();

    println!("init called!");
}

pub fn shutdown(interface: &mut dyn IInterface) {
    // example of what you might want to do with an interface
    interface.dec_counter();

    println!("shutdown called!");
}

plugin!(
    "plugin_example",
    &["crwn1337", "another author"],
    Some(init),
    Some(shutdown),
    State { m_test: 1337 }
);

