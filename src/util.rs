//! Self-contained, re-usable utilities that are not specific to this game.

#![allow(dead_code)]

pub mod config;
pub mod late_despawn;
pub mod old;
pub mod patch;
pub mod selection;

#[allow(unused_imports)]
pub mod prelude {
    pub use bevy::ecs::system::RunSystemOnce as _;
    pub use tiny_bail::prelude::*;

    pub use super::config::Config;
    pub use super::config::ConfigHandle;
    pub use super::config::ConfigRef;
    pub use super::late_despawn::LateDespawn;
    pub use super::old::Old;
    pub use super::patch::prelude::*;
    pub use super::selection::Selection;
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((late_despawn::plugin, old::plugin, selection::plugin));
}
