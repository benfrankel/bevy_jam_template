//! Self-contained, re-usable utilities that are not specific to this game.

#![allow(dead_code)]

pub mod config;
pub mod late_despawn;
pub mod patch;
pub mod selection;
pub mod time;

#[allow(unused_imports)]
pub mod prelude {
    pub use tiny_bail::prelude::*;

    pub use super::config::Config;
    pub use super::config::ConfigHandle;
    pub use super::late_despawn::LateDespawn;
    pub use super::patch::AppExtConfigure as _;
    pub use super::patch::Configure;
    pub use super::patch::Dir2ExtToQuat as _;
    pub use super::patch::EntityCommandsExtTrigger as _;
    pub use super::patch::EntityWorldMutExtAdd as _;
    pub use super::patch::PluginGroupBuilderExtReplace as _;
    pub use super::patch::SpawnWithExt as _;
    pub use super::patch::TriggerExtGetEntity as _;
    pub use super::patch::WorldSpawnWithExt as _;
    pub use super::selection::Selection;
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((late_despawn::plugin, selection::plugin));
}
