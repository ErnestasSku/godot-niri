use crate::niri::niri_types::*;
use godot::prelude::*;
use niri_ipc;

pub(crate) trait FromNiri<Niri> {
    fn from_niri(niri_type: Niri) -> Gd<Self>
    where
        Self: Sized,
        Self: godot::prelude::GodotClass;
}

impl FromNiri<niri_ipc::Output> for NiriOutput {
    fn from_niri(output: niri_ipc::Output) -> Gd<Self> {
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
                    .map(|mode| NiriMode::from_niri(mode))
                    .collect::<Array<Gd<NiriMode>>>(),
                current_mode: 0,
                is_custom_mode: output.is_custom_mode,
                vrr_supported: output.vrr_supported,
                vrr_enabled: output.vrr_enabled,
                logical: NiriLogicalOutput::from_niri(output.logical.unwrap_or(
                    niri_ipc::LogicalOutput {
                        x: 0,
                        y: 0,
                        width: 0,
                        height: 0,
                        scale: 0.0,
                        transform: niri_ipc::Transform::Normal,
                    },
                )),
                base,
            }
        })
    }
}

impl FromNiri<niri_ipc::LogicalOutput> for NiriLogicalOutput {
    fn from_niri(logical_output: niri_ipc::LogicalOutput) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            x: logical_output.x,
            y: logical_output.y,
            width: logical_output.width,
            height: logical_output.height,
            scale: logical_output.scale,
            transform: match logical_output.transform {
                niri_ipc::Transform::Normal => NiriTransform::Normal,
                niri_ipc::Transform::_90 => NiriTransform::_90,
                niri_ipc::Transform::_180 => NiriTransform::_180,
                niri_ipc::Transform::_270 => NiriTransform::_270,
                niri_ipc::Transform::Flipped => NiriTransform::Flipped,
                niri_ipc::Transform::Flipped90 => NiriTransform::Flipped90,
                niri_ipc::Transform::Flipped180 => NiriTransform::Flipped180,
                niri_ipc::Transform::Flipped270 => NiriTransform::Flipped270,
            },
            base,
        })
    }
}

impl FromNiri<&niri_ipc::Mode> for NiriMode {
    fn from_niri(mode: &niri_ipc::Mode) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            width: mode.width,
            height: mode.height,
            refresh_rate: mode.refresh_rate,
            is_preferred: mode.is_preferred,
            base,
        })
    }
}

impl FromNiri<&niri_ipc::Workspace> for NiriWorkspace {
    fn from_niri(workspace: &niri_ipc::Workspace) -> Gd<Self>
    where
        Self: Sized,
        Self: godot::prelude::GodotClass,
    {
        Gd::from_init_fn(|base| Self {
            id: workspace.id as i64,
            idx: workspace.idx,
            name: workspace.name.clone().unwrap_or_default().to_gstring(),
            output: workspace.output.clone().unwrap_or_default().to_gstring(),
            is_urgent: workspace.is_urgent,
            is_active: workspace.is_active,
            is_focused: workspace.is_focused,
            active_window_id: workspace.active_window_id.unwrap_or_default() as i64,
            base,
        })
    }
}
