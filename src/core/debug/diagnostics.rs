use bevy::diagnostic::DiagnosticsStore;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::prelude::*;

use crate::core::debug::DebugConfig;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        FrameTimeDiagnosticsPlugin::default(),
        // TODO: This is disabled by the `bevy/dynamic_linking` feature.
        SystemInformationDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        LogDiagnosticsPlugin::default(),
    ));

    // Disable all diagnostics by default.
    let mut store = app.world_mut().resource_mut::<DiagnosticsStore>();
    for diagnostic in store.iter_mut() {
        diagnostic.is_enabled = false;
    }
}

/// Enable diagnostics determined by the loaded config.
pub(super) fn on_load(config: &DebugConfig, world: &mut World) {
    // Disable all diagnostics first.
    let mut store = world.resource_mut::<DiagnosticsStore>();
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
    let mut store = world.resource_mut::<DiagnosticsStore>();
    for path in to_enable {
        c!(store.get_mut(&path)).is_enabled = true;
    }
}
