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
    // To make a method known to Godot, use the #[method] attribute.
    // In Godot, script "classes" do not actually inherit the parent class.
    // Instead, they are "attached" to the parent object, called the "base".
    //
    // If access to the base instance is desired, the 2nd parameter can be
    // annotated with #[base]. It must have type `&T` or `TRef<T>`, where `T`
    // is the base type specified in #[inherit]. If you don't need this parameter,
    // feel free to omit it entirely.
    #[method]
    fn _ready(&self, #[base] base: &Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("Hello world from node {}!", base.to_string());
    }
}
// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
