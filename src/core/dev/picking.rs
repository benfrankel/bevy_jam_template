use bevy::dev_tools::picking_debug::DebugPickingMode;
use bevy::dev_tools::picking_debug::DebugPickingPlugin;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(DebugPickingPlugin);
    app.add_systems(
        Update,
        toggle_debug_picking.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::F1;

#[cfg_attr(feature = "native_dev", hot)]
fn toggle_debug_picking(mut mode: ResMut<DebugPickingMode>) {
    *mode = match *mode {
        DebugPickingMode::Disabled => DebugPickingMode::Normal,
        _ => DebugPickingMode::Disabled,
    };
}
