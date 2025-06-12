use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Set up physics debug rendering.
    app.add_plugins(PhysicsDebugPlugin::default());
    app.insert_gizmo_config(
        PhysicsGizmos::default(),
        GizmoConfig {
            enabled: false,
            ..default()
        },
    );
    app.add_systems(
        Update,
        toggle_physics_debug_render.run_if(input_just_pressed(TOGGLE_KEY)),
    );

    // Set up physics diagnostics UI.
    app.add_plugins((PhysicsDiagnosticsPlugin, PhysicsDiagnosticsUiPlugin));
    app.insert_resource(PhysicsDiagnosticsUiSettings {
        enabled: false,
        ..default()
    });
    app.add_systems(
        Update,
        toggle_physics_diagnostics_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::F2;

fn toggle_physics_debug_render(mut gizmos: ResMut<GizmoConfigStore>) {
    gizmos.config_mut::<PhysicsGizmos>().0.enabled ^= true;
}

fn toggle_physics_diagnostics_ui(mut settings: ResMut<PhysicsDiagnosticsUiSettings>) {
    settings.enabled ^= true;
}
