include!("../../definitions/src/lib.rs");

example_callback!(example_callback, example_struct, {
    example_struct.m_age = 23;
    example_struct.print();
    true
});

interface!(init, interface, {
    interface.add_callback(example_callback);
    println!("added callback(s) from the dll!");
});

interface!(shutdown, interface, {
    interface.remove_callback(example_callback);
    println!("removed callback(s) from the dll!");
});

plugin!(
    "plugin_example",
    &["crwn1337", "another author"],
    Some(init),
    Some(shutdown)
);
