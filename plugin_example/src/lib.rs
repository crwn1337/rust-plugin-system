include!("../../definitions/src/lib.rs");

pub fn example_callback(example_struct: &mut dyn IExample) -> bool {
    example_struct.as_mut().m_age = 23;
    example_struct.print();
    true
}

pub fn init(interface: &mut dyn IInterface) {
    interface.add_callback(example_callback);
    println!("added callback(s) from the dll!");
}

pub fn shutdown(interface: &mut dyn IInterface) {
    interface.remove_callback(example_callback);
    println!("removed callback(s) from the dll!");
}

declare_plugin!(
    "plugin_example",
    &["crwn1337", "another author"],
    Some(init),
    Some(shutdown)
);
