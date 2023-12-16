#![allow(unused_imports)]

mod font;
mod interaction_palette;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub use crate::ui::font::FontSize;
pub use crate::ui::font::BOLD_FONT_HANDLE;
pub use crate::ui::font::FONT_HANDLE;
pub use crate::ui::interaction_palette::Disabled;
pub use crate::ui::interaction_palette::InteractionPalette;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPickingPlugins,
            font::FontPlugin,
            interaction_palette::InteractionPalettePlugin,
        ));
    }
}
