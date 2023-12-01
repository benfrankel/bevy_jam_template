use bevy::core::FrameCount;
use bevy::diagnostic::EntityCountDiagnosticsPlugin;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::diagnostic::SystemInformationDiagnosticsPlugin;
use bevy::ecs::schedule::LogLevel;
use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::render::DebugRenderContext;
use bevy_rapier2d::render::RapierDebugRenderPlugin;
use iyes_progress::prelude::*;
use strum::IntoEnumIterator;

use crate::state::AppState;

pub struct DebugPlugin {
    pub frame_time_diagnostics: bool,
    pub system_information_diagnostics: bool,
    pub entity_count_diagnostics: bool,
    pub ambiguity_detection: bool,
    pub debug_render: bool,
    pub editor: bool,
    pub start: AppState,
    pub extend_loading_screen: f32,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // Diagnostics
        if self.frame_time_diagnostics {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }
        if self.system_information_diagnostics {
            app.add_plugins(SystemInformationDiagnosticsPlugin);
        }
        if self.entity_count_diagnostics {
            app.add_plugins(EntityCountDiagnosticsPlugin);
        }

        // Logging
        app.add_plugins(LogDiagnosticsPlugin::default());
        if self.ambiguity_detection {
            for (_, schedule) in app.world.resource_mut::<Schedules>().iter_mut() {
                schedule.set_build_settings(ScheduleBuildSettings {
                    ambiguity_detection: LogLevel::Warn,
                    ..default()
                });
            }
        }
        for state in AppState::iter() {
            app.add_systems(OnEnter(state), move |frame: Res<FrameCount>| {
                info!("[Frame {}] Entering {state:?}", frame.0)
            })
            .add_systems(OnExit(state), move |frame: Res<FrameCount>| {
                info!("[Frame {}] Exiting {state:?}", frame.0)
            });
        }

        // Debug render
        if self.debug_render {
            app.add_plugins(RapierDebugRenderPlugin::default());
            app.world.resource_mut::<DebugRenderContext>().enabled = false;
            app.add_systems(
                Update,
                toggle_debug_render.run_if(input_just_pressed(DEBUG_RENDER_TOGGLE_KEY)),
            );
        }

        // Extend loading screen
        if self.extend_loading_screen > 0.0 {
            let delay = self.extend_loading_screen;
            let fake_task = move |mut start: Local<f32>, time: Res<Time>| -> Progress {
                let elapsed = time.elapsed_seconds();
                if *start == 0.0 {
                    *start = elapsed;
                }
                (elapsed - *start >= delay).into()
            };
            app.add_systems(
                Update,
                (
                    (|| Progress::from(false))
                        .track_progress()
                        .run_if(in_state(AppState::TitleScreen)),
                    fake_task
                        .track_progress()
                        .run_if(in_state(AppState::LoadingScreen)),
                ),
            );
        }

        // Skip to custom start state
        *app.world.resource_mut::<State<AppState>>() = State::new(self.start);

        // Editor
        if self.editor {
            app.add_plugins(EditorPlugin::new().in_new_window(Window {
                mode: WindowMode::BorderlessFullscreen,
                title: "bevy_editor_pls".to_string(),
                focused: false,
                ..default()
            }));
        }

        app.add_systems(Update, debug_start);
        app.add_systems(Update, debug_end);
    }
}

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

impl Default for DebugPlugin {
    fn default() -> Self {
        Self {
            frame_time_diagnostics: true,
            system_information_diagnostics: true,
            entity_count_diagnostics: true,
            ambiguity_detection: true,
            debug_render: true,
            editor: true,
            extend_loading_screen: 0.0,
            start: default(),
        }
    }
}

const DEBUG_RENDER_TOGGLE_KEY: KeyCode = KeyCode::F3;

fn toggle_debug_render(mut debug_render_context: ResMut<DebugRenderContext>) {
    debug_render_context.enabled = !debug_render_context.enabled;
}
