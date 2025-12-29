use godot::classes::Engine;
use godot::prelude::*;

struct GodotNiri;

#[gdextension]
unsafe impl ExtensionLibrary for GodotNiri {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            godot_print!("Registering: {}", NiriIPC::class_id().to_string_name());
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

#[derive(GodotClass)]
#[class(base=Object)]
struct NiriIPC {
    commands_socket: Option<niri_ipc::socket::Socket>,
    _event_stream_socket: Option<niri_ipc::socket::Socket>,
    base: Base<Object>,
}

#[godot_api]
impl IObject for NiriIPC {
    fn init(base: Base<Object>) -> Self {
        let commands_socket = niri_ipc::socket::Socket::connect()
            .inspect_err(|_| {
                godot_error!("Failed to connect to niri socket");
            })
            .ok();

        NiriIPC {
            commands_socket,
            _event_stream_socket: None,
            base: base,
        }
    }
}

use niri_ipc::Response::*;

#[godot_api]
impl NiriIPC {
    #[func]
    fn get_outputs(&mut self) -> GString {
        if let Some(socket) = &mut self.commands_socket {
            let outputs = socket.send(niri_ipc::Request::Outputs);

            match outputs {
                Ok(out) => {
                    match out {
                        Ok(response) => match response {
                            Outputs(a) => {
                                for (s, o) in a.iter() {
                                    godot_print!("A output: {}", s);
                                }
                            }
                            _ => {}
                        },
                        Err(err) => {
                            godot_error!("niri output error: %{}", err);
                        }
                    }

                    GString::new()
                }
                Err(_ere) => {
                    godot_error!("Failed to get outputs");

                    GString::new()
                }
            }
        } else {
            GString::new()
        }
    }
}
