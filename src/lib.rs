use godot::{classes::class_db, prelude::*};

struct GodotNiri;

#[gdextension]
unsafe impl ExtensionLibrary for GodotNiri {}
