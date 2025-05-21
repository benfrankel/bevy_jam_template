//! Dev tools for dev builds.

mod diagnostics;
mod editor;
#[cfg(feature = "native_dev")]
mod hot_patch;
mod physics;
mod picking;
mod state;
mod ui;

use crate::prelude::*;
use crate::screen::Screen;

pub(super) fn plugin(app: &mut App) {
    app.configure::<ConfigHandle<DevConfig>>();

    app.add_plugins((
        diagnostics::plugin,
        editor::plugin,
        #[cfg(feature = "native_dev")]
        hot_patch::plugin,
        physics::plugin,
        picking::plugin,
        state::plugin,
        ui::plugin,
    ));

    // Apply the default config.
    DevConfig::default().on_load(app.world_mut());

    // Set up ad hoc debugging.
    app.add_systems(Update, debug_start);
    app.add_systems(Update, debug_end);
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
struct DevConfig {
    // Diagnostics:
    pub log_frame_time: bool,
    pub log_system_information: bool,
    pub log_entity_count: bool,

    // State:
    pub log_state_flush: bool,
    pub extend_loading_screen: f32,
    pub initial_screen: Option<Screen>,
}

impl Default for DevConfig {
    fn default() -> Self {
        Self {
            log_frame_time: false,
            log_system_information: false,
            log_entity_count: false,

            log_state_flush: true,
            extend_loading_screen: 0.0,
            initial_screen: None,
        }
    }
}

impl Config for DevConfig {
    const FILE: &'static str = ".dev.ron";

    fn on_load(&self, world: &mut World) {
        diagnostics::on_load(self, world);
        state::on_load(self, world);
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn debug_start(world: &mut World) {
    let frame = r!(world.get_resource::<FrameCount>()).0;
    let prefix = format!("[Frame {frame} start] ");
    let _ = prefix;
}

#[cfg_attr(feature = "native_dev", hot)]
fn debug_end(world: &mut World) {
    let frame = r!(world.get_resource::<FrameCount>()).0;
    let prefix = format!("[Frame {frame} end] ");
    let _ = prefix;
}
