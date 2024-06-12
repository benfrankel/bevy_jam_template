// Disable common false-positive clippy warnings
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod core;
mod game;
mod sequence;
mod util;

use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            core::CorePlugin,
            game::GamePlugin,
            sequence::SequencePlugin,
            util::UtilPlugin,
        ));
    }
}
