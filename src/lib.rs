mod animation;
mod core;
mod menu;
mod screen;
mod theme;
mod util;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        core::plugin,
        menu::plugin,
        screen::plugin,
        theme::plugin,
        util::plugin,
    ));
}
