#![allow(unused_imports)]
#![allow(dead_code)]

mod font;
mod interaction;
mod tooltip;
mod transition;

use bevy::prelude::*;

pub use crate::ui::font::parse_rich;
pub use crate::ui::font::parse_rich_custom;
pub use crate::ui::font::FontSize;
pub use crate::ui::font::BOLD_FONT_HANDLE;
pub use crate::ui::font::FONT_HANDLE;
pub use crate::ui::font::THICK_FONT_HANDLE;
pub use crate::ui::interaction::InteractionPalette;
pub use crate::ui::interaction::IsDisabled;
pub use crate::ui::tooltip::Tooltip;
pub use crate::ui::tooltip::TooltipSide;
pub use crate::ui::transition::fade_in;
pub use crate::ui::transition::fade_out;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            font::FontPlugin,
            interaction::InteractionPlugin,
            tooltip::TooltipPlugin,
            transition::TransitionPlugin,
        ));
    }
}
