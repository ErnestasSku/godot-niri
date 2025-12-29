use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object)]
pub struct NiriIPC {
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

#[godot_api]
impl NiriIPC {
    #[func]
    fn get_version(&mut self) -> GString {
        if let Some(socket) = &mut self.commands_socket {
            let response = socket.send(niri_ipc::Request::Version);
            let response = response.unwrap().unwrap();

            if let niri_ipc::Response::Version(version) = response {
                return GString::from(version.as_str());
            }
        }
        GString::new()
    }

    #[func]
    fn get_outputs() -> VarDictionary {
        todo!()
    }

    #[func]
    fn get_workspaces() -> Vec<GString> {
        todo!()
    }

    // #[func]
    // fn get_outputs(&mut self) -> GString {
    //     if let Some(socket) = &mut self.commands_socket {
    //         let outputs = socket.send(niri_ipc::Request::Outputs);

    //         match outputs {
    //             Ok(out) => {
    //                 match out {
    //                     Ok(response) => match response {
    //                         Outputs(a) => {
    //                             for (s, o) in a.iter() {
    //                                 godot_print!("A output: {}", s);
    //                             }
    //                         }
    //                         _ => {}
    //                     },
    //                     Err(err) => {
    //                         godot_error!("niri output error: %{}", err);
    //                     }
    //                 }

    //                 GString::new()
    //             }
    //             Err(_ere) => {
    //                 godot_error!("Failed to get outputs");

    //                 GString::new()
    //             }
    //         }
    //     } else {
    //         GString::new()
    //     }
    // }
}
