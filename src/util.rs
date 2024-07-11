//! Self-contained, re-usable utilities that are not specific to this game.

#![allow(dead_code, unused_imports)]

pub mod animation;
pub mod despawn;
pub mod patch;
pub mod time;

pub mod prelude {
    pub use super::despawn::DespawnSet;
    pub use super::patch::AppExtConfigure as _;
    pub use super::patch::Configure;
    pub use super::patch::EntityWorldMutExtAdd as _;
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, despawn::plugin));
}
