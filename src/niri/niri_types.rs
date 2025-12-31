use godot::prelude::*;

pub trait ToGString {
    fn to_gstring(&self) -> GString;
}

impl ToGString for String {
    fn to_gstring(&self) -> GString {
        GString::from(self.as_str())
    }
}

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
    pub modes: Array<Gd<NiriMode>>,
    #[var]
    pub current_mode: i32,
    #[var]
    pub is_custom_mode: bool,
    #[var]
    pub vrr_supported: bool,
    #[var]
    pub vrr_enabled: bool,
    #[var]
    pub logical: Gd<NiriLogicalOutput>,
    pub base: Base<RefCounted>,
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct NiriMode {
    #[var]
    pub width: u16,
    #[var]
    pub height: u16,
    #[var]
    pub refresh_rate: u32,
    #[var]
    pub is_preferred: bool,

    pub base: Base<RefCounted>,
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct NiriLogicalOutput {
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
    pub transform: NiriTransform,
    pub base: Base<RefCounted>,
}

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum NiriTransform {
    Normal,
    _90,
    _180,
    _270,
    Flipped,
    Flipped90,
    Flipped180,
    Flipped270,
}

#[derive(GodotClass)]
#[class(no_init)]
pub struct NiriWorkspace {
    #[var]
    pub id: i64,
    #[var]
    pub idx: u8,
    #[var]
    pub name: GString,
    #[var]
    pub output: GString,
    #[var]
    pub is_urgent: bool,
    #[var]
    pub is_active: bool,
    #[var]
    pub is_focused: bool,
    #[var]
    pub active_window_id: i64,

    pub base: Base<RefCounted>,
}
