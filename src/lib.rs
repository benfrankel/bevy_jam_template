mod animation;
mod core;
mod menu;
mod screen;
mod theme;
mod util;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    // Add core plugins.
    app.add_plugins(core::plugin);

    // Add other plugins.
    app.add_plugins((
        animation::plugin,
        menu::plugin,
        screen::plugin,
        theme::plugin,
        util::plugin,
    ));
}
