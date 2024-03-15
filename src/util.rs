//! Re-usable utilities that could be used in other games

#![allow(dead_code)]
#![allow(unused_imports)]

pub use crate::util::despawn::DespawnSet;
pub use crate::util::time::wait;

pub mod animation;
mod despawn;
mod time;
pub mod ui;

use bevy::prelude::*;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            animation::AnimationPlugin,
            despawn::DespawnPlugin,
            ui::UiPlugin,
        ));
    }
}
