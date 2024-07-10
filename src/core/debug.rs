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
use bevy_mod_picking::debug::DebugPickingMode;
//use bevy_editor_pls::EditorPlugin;
use iyes_progress::prelude::*;
use strum::IntoEnumIterator;

use crate::screen::Screen;
use crate::util::time::wait;

pub(super) struct DebugPlugin {
    // Diagnostics
    pub frame_time_diagnostics: bool,
    pub system_information_diagnostics: bool,
    pub entity_count_diagnostics: bool,

    // Logging
    pub log_diagnostics: bool,
    pub log_ambiguity_detection: bool,
    pub log_screen_transitions: bool,

    // 3rd-party debug tools
    pub debug_picking: bool,
    pub debug_physics: bool,
    //pub editor: bool,

    // Screen settings
    pub start_screen: Screen,
    pub extend_loading_screen: f32,
}

impl Default for DebugPlugin {
    fn default() -> Self {
        Self {
            frame_time_diagnostics: true,
            system_information_diagnostics: true,
            entity_count_diagnostics: true,

            log_diagnostics: true,
            log_ambiguity_detection: true,
            log_screen_transitions: true,

            debug_picking: true,
            debug_physics: true,
            //editor: true,
            //
            extend_loading_screen: 0.0,
            start_screen: default(),
        }
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Collect diagnostics.
        if self.frame_time_diagnostics {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }
        if self.system_information_diagnostics {
            app.add_plugins(SystemInformationDiagnosticsPlugin);
        }
        if self.entity_count_diagnostics {
            app.add_plugins(EntityCountDiagnosticsPlugin);
        }

        // Log diagnostics.
        if self.log_diagnostics {
            app.add_plugins(LogDiagnosticsPlugin::default());
        }

        // Log ambiguity detection results.
        if self.log_ambiguity_detection {
            for (_, schedule) in app.world_mut().resource_mut::<Schedules>().iter_mut() {
                schedule.set_build_settings(ScheduleBuildSettings {
                    ambiguity_detection: LogLevel::Warn,
                    ..default()
                });
            }
        }

        // Log screen transitions.
        if self.log_screen_transitions {
            for screen in Screen::iter() {
                app.add_systems(OnEnter(screen), move |frame: Res<FrameCount>| {
                    info!("[Frame {}] Entering {screen:?}", frame.0)
                });
                app.add_systems(OnExit(screen), move |frame: Res<FrameCount>| {
                    info!("[Frame {}] Exiting {screen:?}", frame.0)
                });
            }
        }

        // Debug picking.
        if self.debug_picking {
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
        if self.debug_physics {
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
        /*if self.editor {
            app.add_plugins(EditorPlugin::new().in_new_window(Window {
                title: "bevy_editor_pls".to_string(),
                focused: false,
                ..default()
            }));
        }*/

        // Extend loading screen.
        if self.extend_loading_screen > 0.0 {
            app.add_systems(
                Update,
                (
                    (|| Progress::from(false))
                        .track_progress()
                        .run_if(in_state(Screen::Title)),
                    wait(self.extend_loading_screen).run_if(in_state(Screen::Loading)),
                ),
            );
        }

        // Skip to a custom starting screen.
        // Setting this at startup instead of right now prevents a plugin ordering requirement
        app.add_systems(Startup, {
            let start = self.start_screen;
            move |mut screen: ResMut<State<_>>| {
                *screen = State::new(start);
            }
        });

        // Set up ad hoc debugging.
        app.add_systems(Update, debug_start);
        app.add_systems(Update, debug_end);
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
