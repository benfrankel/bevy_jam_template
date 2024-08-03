use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_mod_picking::debug::DebugPickingMode;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        toggle_debug_picking.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::F3;

fn toggle_debug_picking(mut mode: ResMut<DebugPickingMode>) {
    *mode = match *mode {
        DebugPickingMode::Disabled => DebugPickingMode::Normal,
        _ => DebugPickingMode::Disabled,
    };
}
