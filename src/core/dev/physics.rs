use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsDebugPlugin::default());
    app.add_systems(
        Update,
        toggle_debug_physics.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    // Disable debug physics by default.
    r!(app.world_mut().get_resource_mut::<GizmoConfigStore>())
        .config_mut::<PhysicsGizmos>()
        .0
        .enabled = false;
}

const TOGGLE_KEY: KeyCode = KeyCode::F3;

fn toggle_debug_physics(mut gizmos: ResMut<GizmoConfigStore>) {
    gizmos.config_mut::<PhysicsGizmos>().0.enabled ^= true;
}
