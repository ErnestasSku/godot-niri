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
    match resp {
        Ok(Ok(r)) => Ok(r),
        Ok(Err(protocol_err)) => Err(protocol_err),
        Err(io_err) => Err(io_err.to_string()),
    }
}

fn _extract_variant<R>(
    response: Result<Response, String>,
    extractor: impl FnOnce(Response) -> Option<R>,
) -> Result<R, String> {
    response.and_then(|resp| extractor(resp).ok_or_else(|| "Unexpected response variant".into()))
}

#[derive(GodotClass)]
#[class(no_init)]
struct NiriOutput {
    #[var]
    pub name: GString,
    #[var]
    pub make: GString,
    #[var]
    pub model: GString,
    #[var]
    pub serial: GString,
    #[var]
    pub physical_size: PackedInt32Array,
    #[var]
    pub modes: PackedInt32Array, // TODO: implement mode
    #[var]
    pub current_mode: i32,
    #[var]
    pub is_custom_mode: bool,
    #[var]
    pub vrr_supported: bool,
    #[var]
    pub vrr_enabled: bool,
    #[var]
    pub logical: bool, // TODO: implement LogicalOutput

    base: Base<RefCounted>,
}

#[godot_api]
impl NiriIPC {
    #[func]
    fn get_version(&mut self) -> GString {
        self.commands_socket
            .as_mut()
            .map_or(GString::default(), |socket| {
                let raw_resp = socket.send(Request::Version);
                let resp = normalize_response(raw_resp);

                if let Ok(Response::Version(version)) = resp {
                    GString::from(version.as_str())
                } else {
                    GString::default()
                }
            })
    }

    #[func]
    fn get_outputs(&mut self) -> VarDictionary {
        self.commands_socket
            .as_mut()
            .map_or(VarDictionary::default(), |socket| {
                let raw_resp = socket.send(Request::Outputs);
                let resp = normalize_response(raw_resp);

                if let Ok(Response::Outputs(outputs)) = resp {
                    let mut dictionary = VarDictionary::default();

                    for (name, output) in outputs {
                        // TODO:
                        let model = GString::from(output.model.as_str());
                        let _ = dictionary.insert(GString::from(name.as_str()), model);
                    }

                    dictionary
                } else {
                    VarDictionary::default()
                }
            })
    }

    #[func]
    fn get_workspaces() -> Vec<GString> {
        todo!()
    }
}
