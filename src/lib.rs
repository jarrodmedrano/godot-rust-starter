use gdnative::prelude::*;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    // Register the new `HelloWorld` type we just declared.
    handle.add_class::<HelloWorld>();
}
// You may add any number of ordinary `impl` blocks as you want. However, ...
impl HelloWorld {
    /// The "constructor" of the class.
    fn new(_base: &Node) -> Self {
        HelloWorld
    }
}
// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {
}
// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
