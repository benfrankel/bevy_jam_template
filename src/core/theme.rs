use std::ops::Index;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumCount;

use crate::core::config::Config;
use crate::core::config::ConfigHandle;
use crate::core::UpdateSet;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ThemeSpriteColor>()
            .add_systems(Update, apply_theme_sprite_color.in_set(UpdateSet::End));

        app.register_type::<ThemeUiImageColor>()
            .add_systems(Update, apply_theme_ui_image_color.in_set(UpdateSet::End));

        app.register_type::<ThemeTextColors>()
            .add_systems(Update, apply_theme_text_colors.in_set(UpdateSet::End));

        app.register_type::<ThemeBackgroundColor>()
            .add_systems(Update, apply_theme_background_color.in_set(UpdateSet::End));

        app.register_type::<ThemeBorderColor>()
            .add_systems(Update, apply_theme_border_color.in_set(UpdateSet::End));
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

/// See: https://getbootstrap.com/docs/5.3/customize/color/
#[derive(Reflect, Clone, Copy, Default, EnumCount)]
pub enum ThemeColor {
    // Absolute colors
    #[default]
    None,

    // Semantic colors
    Body,
    BodyText,

    Primary,
    PrimaryHovered,
    PrimaryPressed,
    PrimaryDisabled,
    PrimaryText,

    // Misc UI colors
    Popup,
}

#[derive(Component, Reflect, Default)]
pub struct ThemeSpriteColor(pub ThemeColor);

fn apply_theme_sprite_color(
    config_handle: Res<ConfigHandle>,
    config: Res<Assets<Config>>,
    mut theme_query: Query<(&ThemeSpriteColor, &mut Sprite)>,
) {
    let Some(palette) = &config
        .get(&config_handle.0)
        .map(|config| &config.theme.colors)
    else {
        return;
    };

    for (color, mut sprite) in &mut theme_query {
        sprite.color = palette[color.0];
    }
}

#[derive(Component, Reflect, Default)]
pub struct ThemeUiImageColor(pub ThemeColor);

fn apply_theme_ui_image_color(
    config_handle: Res<ConfigHandle>,
    config: Res<Assets<Config>>,
    mut theme_query: Query<(&ThemeUiImageColor, &mut UiImage)>,
) {
    let Some(palette) = &config
        .get(&config_handle.0)
        .map(|config| &config.theme.colors)
    else {
        return;
    };

    for (color, mut image) in &mut theme_query {
        image.color = palette[color.0];
    }
}

#[derive(Component, Reflect, Default)]
pub struct ThemeTextColors(pub Vec<ThemeColor>);

fn apply_theme_text_colors(
    config_handle: Res<ConfigHandle>,
    config: Res<Assets<Config>>,
    mut theme_query: Query<(&ThemeTextColors, &mut Text)>,
) {
    let Some(palette) = &config
        .get(&config_handle.0)
        .map(|config| &config.theme.colors)
    else {
        return;
    };

    for (colors, mut text) in &mut theme_query {
        for (section, &color) in text.sections.iter_mut().zip(&colors.0) {
            section.style.color = palette[color];
        }
    }
}

#[derive(Component, Reflect, Default)]
pub struct ThemeBackgroundColor(pub ThemeColor);

fn apply_theme_background_color(
    config_handle: Res<ConfigHandle>,
    config: Res<Assets<Config>>,
    mut theme_query: Query<(&ThemeBackgroundColor, &mut BackgroundColor)>,
) {
    let Some(palette) = &config
        .get(&config_handle.0)
        .map(|config| &config.theme.colors)
    else {
        return;
    };

    for (color, mut background) in &mut theme_query {
        background.0 = palette[color.0];
    }
}

#[derive(Component, Reflect, Default)]
pub struct ThemeBorderColor(pub ThemeColor);

fn apply_theme_border_color(
    config_handle: Res<ConfigHandle>,
    config: Res<Assets<Config>>,
    mut theme_query: Query<(&ThemeBorderColor, &mut BorderColor)>,
) {
    let Some(palette) = &config
        .get(&config_handle.0)
        .map(|config| &config.theme.colors)
    else {
        return;
    };

    for (color, mut border) in &mut theme_query {
        border.0 = palette[color.0];
    }
}
