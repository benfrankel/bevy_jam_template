mod font;

use bevy::prelude::*;

pub use crate::ui::font::FONT_HANDLE;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Selection>()
            .add_plugins(font::FontPlugin);
    }
}

#[derive(Component, Reflect)]
pub struct Selection(pub Entity);

impl Default for Selection {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}
