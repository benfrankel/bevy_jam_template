mod animation;
mod core;
mod game;
mod screen;
mod ui;
mod util;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        core::plugin,
        game::plugin,
        screen::plugin,
        ui::plugin,
        util::plugin,
    ));
}
