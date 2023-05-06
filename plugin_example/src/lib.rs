include!("../../definitions/src/lib.rs");

pub fn example_callback(example: &mut dyn IExample) -> bool {
    example.m_age = 23;
    example.print();
    true
}

pub fn init(interface: &mut dyn IInterface) {
    interface.add_callback(example_callback);
    println!("added callback(s) from the dll!");
}

pub fn shutdown(interface: &mut dyn IInterface) {
    interface.remove_callback(example_callback);
    println!("added callback(s) from the dll!");
}

plugin!(
    "plugin_example",
    &["crwn1337", "another author"],
    Some(init),
    Some(shutdown)
);
