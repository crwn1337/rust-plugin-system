include!("../../definitions/src/lib.rs");

pub struct State {
    pub test_state: u32,
}

impl IPlugin for State {
    fn callback(&mut self, example: &mut dyn IExample) -> bool {
        example.m_age = 23;
        println!("{}", self.test_state);
        example.print();
        true
    }
}

pub fn init(_: &mut dyn IInterface) {
    println!("init called!");
}

pub fn shutdown(_: &mut dyn IInterface) {
    println!("shutdown called!");
}

plugin!(
    "plugin_example",
    &["crwn1337", "another author"],
    Some(init),
    Some(shutdown),
    State { test_state: 0 }
);
