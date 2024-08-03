//! Debugging tools for dev builds.

mod ambiguity;
mod diagnostics;
mod editor;
mod physics;
mod picking;
mod state;
mod ui;

use bevy::core::FrameCount;
use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<DebugConfig>>();

    // Add debug subsystems.
    app.add_plugins((
        diagnostics::plugin,
        editor::plugin,
        physics::plugin,
        picking::plugin,
        state::plugin,
        ui::plugin,
    ));

    // Load the default configs.
    DebugConfig::default().on_load(app.world_mut());

    // Set up ad hoc debugging.
    app.add_systems(Update, debug_start);
    app.add_systems(Update, debug_end);
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
struct DebugConfig {
    // Diagnostics:
    pub frame_time_diagnostics: bool,
    pub system_information_diagnostics: bool,
    pub entity_count_diagnostics: bool,

    // Ambiguity:
    pub log_ambiguity_detection: bool,

    // State:
    pub log_state_flush: bool,
    pub initial_screen: Option<Screen>,
    pub extend_loading_screen: f32,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            frame_time_diagnostics: false,
            system_information_diagnostics: false,
            entity_count_diagnostics: false,

            log_ambiguity_detection: false,

            log_state_flush: true,
            extend_loading_screen: 0.0,
            initial_screen: None,
        }
    }
}

impl Config for DebugConfig {
    const FILE: &'static str = "debug.ron";

    fn on_load(&mut self, world: &mut World) {
        ambiguity::on_load(self, world);
        diagnostics::on_load(self, world);
        state::on_load(self, world);
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
