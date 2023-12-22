#![allow(unused_imports)]

mod despawn;
mod time;

use bevy::prelude::*;

pub use crate::util::despawn::DespawnSet;
pub use crate::util::time::wait;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(despawn::DespawnPlugin);
    }
}
