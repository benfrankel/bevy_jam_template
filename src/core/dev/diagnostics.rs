use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;

use crate::core::dev::DevConfig;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        FrameTimeDiagnosticsPlugin::default(),
        // Note: This is disabled by the `bevy/dynamic_linking` feature.
        SystemInformationDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        LogDiagnosticsPlugin::default(),
    ));

    // Disable all diagnostics by default.
    let mut store = r!(app.world_mut().get_resource_mut::<DiagnosticsStore>());
    for diagnostic in store.iter_mut() {
        diagnostic.is_enabled = false;
    }
}

/// Enable diagnostics determined by the loaded config.
pub(super) fn on_load(config: &DevConfig, world: &mut World) {
    // Disable all diagnostics first.
    let mut store = r!(world.get_resource_mut::<DiagnosticsStore>());
    for diagnostic in store.iter_mut() {
        diagnostic.is_enabled = false;
    }

    let mut to_enable = vec![];

    if config.frame_time_diagnostics {
        to_enable.extend([
            FrameTimeDiagnosticsPlugin::FPS,
            FrameTimeDiagnosticsPlugin::FRAME_COUNT,
            FrameTimeDiagnosticsPlugin::FRAME_TIME,
        ]);
    }

    if config.system_information_diagnostics {
        to_enable.extend([
            SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE,
            SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE,
        ]);
    }

    if config.entity_count_diagnostics {
        to_enable.push(EntityCountDiagnosticsPlugin::ENTITY_COUNT);
    }

    // Re-enable the configured diagnostics.
    for path in to_enable {
        c!(store.get_mut(&path)).is_enabled = true;
    }
}
