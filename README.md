# rust-plugin-system
a simple example for creating a plugin system in Rust without dependencies (except for [libloading](https://crates.io/crates/libloading) for obvious reasons)

### [application](/application/)
the main application (where the plugins get loaded)

### [definitions](/definitions/)
definitions to be used for plugins and the application

### [plugin_example](/plugin_example/)
an example plugin

## pros
* basically 0 overhead (since its pure rust)
* you get to use the entire rust standard library and also any external crates

## cons
* not ffi safe (you should use [abi_stable](https://crates.io/crates/abi_stable), but i don't like dragging in 50~70 dependencies)
* no intellisense (rust-analyzer doesn't like working with the `include!` macro)

## todo:
- [ ] expand this (make a more complicated example)
