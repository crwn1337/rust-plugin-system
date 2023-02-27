include!("../../definitions/src/lib.rs");

pub fn contains_hello(example: &dyn IExample) -> bool {
    example.print();
    true
}

pub fn init(interface: &mut dyn IInterface) {
    interface.add_callback(contains_hello);
    println!("added callback(s) from the dll!");
}

pub fn shutdown(interface: &mut dyn IInterface) {
    interface.remove_callback(contains_hello);
    println!("removed callback(s) from the dll!");
}

declare_plugin!(
    "plugin_example",
    &["crwn1337", "another author"],
    Some(init),
    Some(shutdown)
);
