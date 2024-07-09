//! Self-contained, re-usable utilities that are not specific to this game

#![allow(dead_code)]
#![allow(unused_imports)]

pub use crate::util::despawn::DespawnSet;
pub use crate::util::time::wait;

pub mod animation;
mod despawn;
mod time;
pub mod ui;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, despawn::plugin, ui::plugin));
}
