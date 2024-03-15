#![allow(unused_imports)]
#![allow(dead_code)]

pub use font::parse_rich;
pub use font::parse_rich_custom;
pub use font::FontSize;
pub use font::BOLD_FONT_HANDLE;
pub use font::FONT_HANDLE;
pub use font::THICK_FONT_HANDLE;
pub use interaction::InteractionPalette;
pub use interaction::IsDisabled;
pub use tooltip::Tooltip;
pub use tooltip::TooltipSide;

mod font;
mod interaction;
mod tooltip;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            font::FontPlugin,
            interaction::InteractionPlugin,
            tooltip::TooltipPlugin,
        ));
    }
}
