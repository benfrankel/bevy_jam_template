//! Self-contained, re-usable utilities that are not specific to this game.

#![allow(dead_code, unused_imports)]

pub mod config;
pub mod despawn;
pub mod macros;
pub mod patch;
pub mod time;

pub mod prelude {
    pub use super::config::Config;
    pub use super::config::ConfigHandle;
    pub use super::despawn::DespawnSet;
    pub use super::patch::AppExtConfigure as _;
    pub use super::patch::Configure;
    pub use super::patch::EntityWorldMutExtAdd as _;
    pub use super::patch::PluginGroupBuilderExtReplace as _;
    pub use super::patch::SpawnWithExt as _;
    pub use super::patch::WorldSpawnWithExt as _;
    pub use crate::c;
    pub use crate::r;
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(despawn::plugin);
}
