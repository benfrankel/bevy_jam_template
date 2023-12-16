#![allow(unused_imports)]

mod despawn;

use bevy::prelude::*;

pub use crate::util::despawn::DespawnSet;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(despawn::DespawnPlugin);
    }
}
