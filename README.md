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
* you get to use the entire rust standard library
* you get to use any external crates rust provides
* the rust compiler should help with any errors at compile time

## cons
* not ffi safe, you should use [abi_stable](https://crates.io/crates/abi_stable), but i don't like dragging in 50~70 dependencies
* no intellisense (rust-analyzer doesn't like working with the `include!` macro)
* [there might be lifetime issues with 'static](https://github.com/nagisa/rust_libloading/issues/46), but it only seems to be an issue if you manually drop the library
* there might be issues with very very old versions of [macos](https://github.com/nagisa/rust_libloading/issues/5) and [linux](https://github.com/nagisa/rust_libloading/issues/41)
* compiling the application and plugin with different versions of the rust compiler MIGHT introduce issues (not ffi safe, yet again)
