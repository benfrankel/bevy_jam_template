use std::ops::Index;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;

use crate::config::Config;
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

// TODO: Split the Component into SpritePaletteColor, TextPaletteColor, BackgroundPaletteColor, etc.
#[derive(Component, Reflect, Clone, Copy)]
pub enum PaletteColor {
    Background,
    Foreground,
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct Palette([Color; 2]);

impl Index<PaletteColor> for Palette {
    type Output = Color;

    fn index(&self, index: PaletteColor) -> &Self::Output {
        &self.0[index as usize]
    }
}

fn apply_palette_color(
    config: Res<Config>,
    mut color_query: Query<
        (
            &PaletteColor,
            Option<&mut Sprite>,
            Option<&mut TextureAtlasSprite>,
            Option<&mut Text>,
        ),
        Changed<PaletteColor>,
    >,
) {
    let palette = &config.theme.palette;
    for (&color, sprite, atlas_sprite, text) in &mut color_query {
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
        }
    }
}
