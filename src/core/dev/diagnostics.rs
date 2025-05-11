use bevy::diagnostic::DiagnosticPath;
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
        // Note: This is incompatible with the `bevy/dynamic_linking` feature.
        #[cfg(not(feature = "dev"))]
        SystemInformationDiagnosticsPlugin,
        EntityCountDiagnosticsPlugin,
        LogDiagnosticsPlugin::filtered(FILTER.to_vec()),
    ));

    // Disable all diagnostics by default.
    let mut store = r!(app.world_mut().get_resource_mut::<DiagnosticsStore>());
    for diagnostic in store.iter_mut() {
        diagnostic.is_enabled = false;
    }
}

// TODO: Workaround for <https://github.com/bevyengine/bevy/issues/19175>.
//       It would be preferable to toggle the logging (not collection) of diagnostics.
const FILTER: [DiagnosticPath; 8] = [
    FrameTimeDiagnosticsPlugin::FPS,
    FrameTimeDiagnosticsPlugin::FRAME_COUNT,
    FrameTimeDiagnosticsPlugin::FRAME_TIME,
    SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE,
    SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE,
    SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE,
    SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE,
    EntityCountDiagnosticsPlugin::ENTITY_COUNT,
];

/// Enable diagnostics determined by the loaded config.
pub(super) fn on_load(config: &DevConfig, world: &mut World) {
    let mut paths = vec![];

    if config.log_frame_time {
        paths.extend([
            FrameTimeDiagnosticsPlugin::FPS,
            FrameTimeDiagnosticsPlugin::FRAME_COUNT,
            FrameTimeDiagnosticsPlugin::FRAME_TIME,
        ]);
    }

    if config.log_system_information {
        paths.extend([
            SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE,
            SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE,
            SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE,
            SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE,
        ]);
    }

    if config.log_entity_count {
        paths.push(EntityCountDiagnosticsPlugin::ENTITY_COUNT);
    }

    // Enable only the configured diagnostics.
    let mut store = r!(world.get_resource_mut::<DiagnosticsStore>());
    for path in FILTER {
        cq!(store.get_mut(&path)).is_enabled = false;
    }
    for path in paths {
        cq!(store.get_mut(&path)).is_enabled = true;
    }
}
