//! Self-contained, re-usable utilities that are not specific to this game.

#![allow(dead_code, unused_imports)]

pub mod animation;
pub mod configure;
pub mod despawn;
pub mod time;

pub mod prelude {
    pub use super::configure::AppExtConfigure as _;
    pub use super::configure::Configure;
    pub use super::despawn::DespawnSet;
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, despawn::plugin));
}
