mod core;
mod screen;
mod theme;
mod util;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((core::plugin, screen::plugin, theme::plugin, util::plugin));
}
