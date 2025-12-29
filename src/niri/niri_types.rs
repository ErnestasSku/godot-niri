use godot::prelude::*;
use niri_ipc::LogicalOutput;

#[derive(GodotClass)]
#[class(no_init)]
pub struct NiriOutput {
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
    pub modes: Array<Gd<NiriOutputMode>>,
    #[var]
    pub current_mode: i32,
    #[var]
    pub is_custom_mode: bool,
    #[var]
    pub vrr_supported: bool,
    #[var]
    pub vrr_enabled: bool,
    #[var]
    pub logical: Gd<NiriOutputLogicalOutput>,
    base: Base<RefCounted>,
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct NiriOutputMode {
    #[var]
    pub width: u16,
    #[var]
    pub height: u16,
    #[var]
    pub refresh_rate: u32,
    #[var]
    pub is_preferred: bool,

    base: Base<RefCounted>,
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct NiriOutputLogicalOutput {
    #[var]
    pub x: i32,
    #[var]
    pub y: i32,
    #[var]
    pub width: u32,
    #[var]
    pub height: u32,
    #[var]
    pub scale: f64,
    #[var]
    pub transform: NiriOutputTransform,
    base: Base<RefCounted>,
}

impl NiriOutputLogicalOutput {
    pub fn from_logical_output(logical_output: niri_ipc::LogicalOutput) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            x: logical_output.x,
            y: logical_output.y,
            width: logical_output.width,
            height: logical_output.height,
            scale: logical_output.scale,
            transform: match logical_output.transform {
                niri_ipc::Transform::Normal => NiriOutputTransform::Normal,
                niri_ipc::Transform::_90 => NiriOutputTransform::_90,
                niri_ipc::Transform::_180 => NiriOutputTransform::_180,
                niri_ipc::Transform::_270 => NiriOutputTransform::_270,
                niri_ipc::Transform::Flipped => NiriOutputTransform::Flipped,
                niri_ipc::Transform::Flipped90 => NiriOutputTransform::Flipped90,
                niri_ipc::Transform::Flipped180 => NiriOutputTransform::Flipped180,
                niri_ipc::Transform::Flipped270 => NiriOutputTransform::Flipped270,
            },
            base,
        })
    }
}

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum NiriOutputTransform {
    Normal,
    _90,
    _180,
    _270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

impl NiriOutputMode {
    pub fn from_mode(mode: &niri_ipc::Mode) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            width: mode.width,
            height: mode.height,
            refresh_rate: mode.refresh_rate,
            is_preferred: mode.is_preferred,
            base,
        })
    }
}

pub trait ToGString {
    fn to_gstring(&self) -> GString;
}

impl ToGString for String {
    fn to_gstring(&self) -> GString {
        GString::from(self.as_str())
    }
}

impl NiriOutput {
    pub fn from_output(output: niri_ipc::Output) -> Gd<Self> {
        Gd::from_init_fn(|base| {
            let physical_size = output
                .physical_size
                .map_or(PackedInt32Array::from([0, 0]), |(x, y)| {
                    PackedInt32Array::from([x as i32, y as i32])
                });

            Self {
                name: output.name.to_gstring(),
                make: output.make.to_gstring(),
                model: output.model.to_gstring(),
                serial: output
                    .serial
                    .map_or(GString::default(), |serial| serial.to_gstring()),
                physical_size,
                modes: output
                    .modes
                    .iter()
                    .map(|mode| NiriOutputMode::from_mode(mode))
                    .collect::<Array<Gd<NiriOutputMode>>>(),
                current_mode: 0,
                is_custom_mode: output.is_custom_mode,
                vrr_supported: output.vrr_supported,
                vrr_enabled: output.vrr_enabled,
                logical: NiriOutputLogicalOutput::from_logical_output(
                    output.logical.unwrap_or(LogicalOutput { x: 0, y: 0, width: 0, height: 0, scale: 0.0, transform: niri_ipc::Transform::Normal })
                ),
                base,
            }
        })
    }
}
