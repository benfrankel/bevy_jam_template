//! Debugging tools for dev builds.

use avian2d::prelude::*;
use bevy::core::FrameCount;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::ecs::schedule::LogLevel;
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_mod_picking::debug::DebugPickingMode;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::core::window::WindowReady;
use crate::screen::Screen;
use crate::util::time::wait;

pub(super) fn plugin(app: &mut App) {
    // TODO: Load from file.
    let config = DebugConfig::default();

    // Collect diagnostics.
    if config.frame_time_diagnostics {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
    }
    if config.system_information_diagnostics {
        app.add_plugins(SystemInformationDiagnosticsPlugin);
    }
    if config.entity_count_diagnostics {
        app.add_plugins(EntityCountDiagnosticsPlugin);
    }

    // Log diagnostics.
    if config.log_diagnostics {
        app.add_plugins(LogDiagnosticsPlugin::default());
    }

    // Log ambiguity detection results.
    if config.log_ambiguity_detection {
        for (_, schedule) in app.world_mut().resource_mut::<Schedules>().iter_mut() {
            schedule.set_build_settings(ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        }
    }

    // Log state flushes.
    if config.log_state_flush {
        app.insert_resource(StateDebugSettings {
            log_flush: true,
            ..default()
        });
    }

    // Debug picking.
    if config.debug_picking {
        app.add_systems(
            Update,
            (|mut mode: ResMut<_>| {
                *mode = match *mode {
                    DebugPickingMode::Disabled => DebugPickingMode::Normal,
                    _ => DebugPickingMode::Disabled,
                };
            })
            .run_if(input_just_pressed(DEBUG_TOGGLE_KEY)),
        );
    }

    // Debug physics.
    if config.debug_physics {
        app.add_plugins(PhysicsDebugPlugin::default());
        app.world_mut()
            .resource_mut::<GizmoConfigStore>()
            .config_mut::<PhysicsGizmos>()
            .0
            .enabled = false;
        app.add_systems(
            Update,
            (|mut gizmos: ResMut<GizmoConfigStore>| {
                gizmos.config_mut::<PhysicsGizmos>().0.enabled ^= true;
            })
            .run_if(input_just_pressed(DEBUG_TOGGLE_KEY)),
        );
    }

    // Enable editor.
    if config.editor {
        app.add_plugins(EditorPlugin::new().in_new_window(Window {
            title: "bevy_editor_pls".to_string(),
            focused: false,
            ..default()
        }));
    }

    // Extend loading screen.
    if config.extend_loading_screen > 0.0 {
        app.add_systems(
            Update,
            (
                Screen::Title.on_update((|| Progress::from(false)).track_progress()),
                Screen::Loading.on_update(wait(config.extend_loading_screen)),
            ),
        );
    }

    // Skip to a custom starting screen.
    if let Some(start_screen) = config.start_screen {
        // Setting this later avoids a plugin ordering requirement.
        app.add_systems(StateFlush, WindowReady.on_enter(start_screen.enter()));
    }

    // Set up ad hoc debugging.
    app.add_systems(Update, debug_start);
    app.add_systems(Update, debug_end);
}

pub(super) struct DebugConfig {
    // Diagnostics
    pub frame_time_diagnostics: bool,
    pub system_information_diagnostics: bool,
    pub entity_count_diagnostics: bool,

    // Logging
    pub log_diagnostics: bool,
    pub log_ambiguity_detection: bool,
    pub log_state_flush: bool,

    // 3rd-party debug tools
    pub debug_picking: bool,
    pub debug_physics: bool,
    pub editor: bool,

    // Screen settings
    pub start_screen: Option<Screen>,
    pub extend_loading_screen: f32,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            frame_time_diagnostics: true,
            system_information_diagnostics: true,
            entity_count_diagnostics: true,

            log_diagnostics: false,
            log_ambiguity_detection: false,
            log_state_flush: true,

            debug_picking: true,
            debug_physics: true,
            editor: true,

            extend_loading_screen: 0.0,
            start_screen: None,
        }
    }
}

const DEBUG_TOGGLE_KEY: KeyCode = KeyCode::F3;

fn debug_start(world: &mut World) {
    let frame = world.resource::<FrameCount>().0;
    let prefix = format!("[Frame {frame} start] ");
    let _ = prefix;
}

fn debug_end(world: &mut World) {
    let frame = world.resource::<FrameCount>().0;
    let prefix = format!("[Frame {frame} end] ");
    let _ = prefix;
}
