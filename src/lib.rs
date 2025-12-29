use godot::classes::Engine;
use godot::prelude::*;

use crate::godot_niri::niri_ipc::NiriIPC;

mod godot_niri;
mod niri;

struct GodotNiri;

#[gdextension]
unsafe impl ExtensionLibrary for GodotNiri {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            Engine::singleton()
                .register_singleton(&NiriIPC::class_id().to_string_name(), &NiriIPC::new_alloc());
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            let niri_ipc_singleton_name = &NiriIPC::class_id().to_string_name();

            if let Some(niri_ipc_singleton) = engine.get_singleton(niri_ipc_singleton_name) {
                engine.unregister_singleton(niri_ipc_singleton_name);
                niri_ipc_singleton.free();
            } else {
                godot_error!("Failed to get niri ipc singleton");
            }
        }
    }
}
