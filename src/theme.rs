use std::ops::Index;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::config::Config;
use crate::config::ConfigHandle;
use crate::AppSet;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PaletteColor>()
            .add_systems(Update, apply_palette_color.in_set(AppSet::End));
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub palette: Palette,
}

impl ThemeConfig {
    pub fn apply(&self, world: &mut World) {
        world.resource_mut::<ClearColor>().0 = self.palette[PaletteColor::Background];
        for mut color in world.query::<&mut PaletteColor>().iter_mut(world) {
            color.set_changed();
        }
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Palette([Color; 2]);

impl Index<PaletteColor> for Palette {
    type Output = Color;

    fn index(&self, index: PaletteColor) -> &Self::Output {
        &self.0[index as usize]
    }
}

/// Applies color to:
/// - Sprite
/// - TextureAtlasSprite
/// - Text (all sections)
/// - BackgroundColor (only when there's no Text component)
#[derive(Component, Reflect, Clone, Copy)]
pub enum PaletteColor {
    Background,
    Foreground,
}

fn apply_palette_color(
    config_handle: Res<ConfigHandle>,
    config: Res<Assets<Config>>,
    mut color_query: Query<
        (
            &PaletteColor,
            Option<&mut Sprite>,
            Option<&mut TextureAtlasSprite>,
            Option<&mut Text>,
            Option<&mut BackgroundColor>,
        ),
        Changed<PaletteColor>,
    >,
) {
    let Some(palette) = &config
        .get(&config_handle.0)
        .map(|config| &config.theme.palette)
    else {
        return;
    };

    for (&color, sprite, atlas_sprite, text, background_color) in &mut color_query {
        let color = palette[color];
        if let Some(mut sprite) = sprite {
            sprite.color = color;
        }
        if let Some(mut atlas_sprite) = atlas_sprite {
            atlas_sprite.color = color;
        }
        if let Some(mut text) = text {
            for section in &mut text.sections {
                section.style.color = color;
            }
        } else if let Some(mut background_color) = background_color {
            background_color.0 = color;
        }
    }
}
