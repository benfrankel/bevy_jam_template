// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]

mod animation;
mod core;
mod menu;
mod prelude;
mod screen;
mod theme;
mod util;

use crate::prelude::*;

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
