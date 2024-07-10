//! Self-contained, re-usable utilities that are not specific to this game

#![allow(dead_code)]
#![allow(unused_imports)]

pub mod animation;
pub mod configure;
pub mod despawn;
pub mod time;
pub mod ui;

pub mod prelude {
    pub use configure::AppExtConfigure as _;
    pub use configure::Configure;
    pub use despawn::DespawnSet;
    pub use ui::UiRoot;
    pub use ui::BOLD_FONT_HANDLE;
    pub use ui::FONT_HANDLE;
    pub use ui::THICK_FONT_HANDLE;

    use super::*;
}

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((animation::plugin, despawn::plugin, ui::plugin));
}
