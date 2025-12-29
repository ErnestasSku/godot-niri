use crate::niri::niri_types::NiriOutput;
use godot::prelude::*;
use niri_ipc::{Request, Response};

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

fn normalize_response(
    resp: Result<Result<Response, String>, std::io::Error>,
) -> Result<Response, String> {
    resp.map_err(|e| e.to_string())?
}

impl NiriIPC {
    #[allow(dead_code)]
    fn with_socket<T>(
        &mut self,
        default: T,
        f: impl FnOnce(&mut niri_ipc::socket::Socket) -> T,
    ) -> T {
        match self.commands_socket.as_mut() {
            Some(socket) => f(socket),
            None => default,
        }
    }

    fn with_response<T>(
        &mut self,
        request: Request,
        default: T,
        f: impl FnOnce(Response) -> Option<T>,
    ) -> T {
        let socket = match self.commands_socket.as_mut() {
            Some(s) => s,
            None => return default,
        };

        let resp = match normalize_response(socket.send(request)) {
            Ok(r) => r,
            Err(_) => return default,
        };

        f(resp).unwrap_or(default)
    }
}

#[godot_api]
impl NiriIPC {
    #[func]
    fn get_version(&mut self) -> GString {
        self.with_response(Request::Version, GString::default(), |resp| match resp {
            Response::Version(v) => Some(GString::from(v.as_str())),
            _ => None,
        })
    }

    #[func]
    fn get_outputs(&mut self) -> VarDictionary {
        self.with_response(
            Request::Outputs,
            VarDictionary::default(),
            |resp| match resp {
                Response::Outputs(o) => {
                    let mut dict = VarDictionary::default();
                    for (name, output) in o {
                        let niri_output = NiriOutput::from_output(output);
                        let _ = dict.insert(name, niri_output);
                    }

                    Some(dict)
                }
                _ => None,
            },
        )
    }

    #[func]
    fn get_workspaces() -> Vec<GString> {
        todo!()
    }
}
