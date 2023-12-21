#![allow(unused_imports)]
#![allow(dead_code)]

mod font;
mod interaction_palette;
mod text;
mod tooltip;

use bevy::prelude::*;
use bevy_mod_picking::prelude::*;

pub use crate::ui::font::FontSize;
pub use crate::ui::font::BOLD_FONT_HANDLE;
pub use crate::ui::font::FONT_HANDLE;
pub use crate::ui::font::THICK_FONT_HANDLE;
pub use crate::ui::interaction_palette::Disabled;
pub use crate::ui::interaction_palette::InteractionPalette;
pub use crate::ui::text::parse_rich;
pub use crate::ui::tooltip::Tooltip;
pub use crate::ui::tooltip::TooltipSide;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPickingPlugins,
            font::FontPlugin,
            interaction_palette::InteractionPalettePlugin,
            tooltip::TooltipPlugin,
        ));
    }
}
