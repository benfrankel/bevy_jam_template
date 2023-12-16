use std::ops::Index;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumCount;

use crate::config::Config;
use crate::config::ConfigHandle;
use crate::AppSet;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ThemeColor>()
            .add_systems(PostUpdate, apply_theme_color.in_set(AppSet::AnimateSync));
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct ThemeConfig {
    pub colors: ThemeColorList,
    // TODO: pub fonts: ThemeFontList,
}

impl ThemeConfig {
    pub fn apply(&self, world: &mut World) {
        world.resource_mut::<ClearColor>().0 = self.colors[ThemeColor::Body];
        for mut color in world.query::<&mut ThemeColor>().iter_mut(world) {
            color.set_changed();
        }
    }
}

#[derive(Reflect, Serialize, Deserialize)]
pub struct ThemeColorList([Color; ThemeColor::COUNT]);

impl Index<ThemeColor> for ThemeColorList {
    type Output = Color;

    fn index(&self, index: ThemeColor) -> &Self::Output {
        &self.0[index as usize]
    }
}

/// Applies color to:
/// - Sprite
/// - TextureAtlasSprite
/// - Text (all sections)
/// - BackgroundColor (only when there's no Text component)
///
/// (see: https://getbootstrap.com/docs/5.3/customize/color/)
#[derive(Component, Reflect, Clone, Copy, EnumCount)]
pub enum ThemeColor {
    None,

    Body,
    BodyText,

    Primary,
    PrimaryHovered,
    PrimaryPressed,
    PrimaryDisabled,
    PrimaryText,

    Popup,
}

fn apply_theme_color(
    config_handle: Res<ConfigHandle>,
    config: Res<Assets<Config>>,
    mut color_query: Query<
        (
            &ThemeColor,
            Option<&mut Sprite>,
            Option<&mut TextureAtlasSprite>,
            Option<&mut Text>,
            Option<&mut BackgroundColor>,
        ),
        Changed<ThemeColor>,
    >,
) {
    let Some(palette) = &config
        .get(&config_handle.0)
        .map(|config| &config.theme.colors)
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
