mod font;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub use crate::ui::font::FontSize;
pub use crate::ui::font::BOLD_FONT_HANDLE;
pub use crate::ui::font::FONT_HANDLE;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DefaultPickingPlugins, font::FontPlugin));
    }
}
