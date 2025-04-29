use avian2d::prelude::*;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsDebugPlugin::default());
    app.add_systems(
        Update,
        toggle_debug_physics.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    // Disable debug physics by default.
    app.world_mut()
        .resource_mut::<GizmoConfigStore>()
        .config_mut::<PhysicsGizmos>()
        .0
        .enabled = false;
}

const TOGGLE_KEY: KeyCode = KeyCode::F3;

fn toggle_debug_physics(mut gizmos: ResMut<GizmoConfigStore>) {
    gizmos.config_mut::<PhysicsGizmos>().0.enabled ^= true;
}
