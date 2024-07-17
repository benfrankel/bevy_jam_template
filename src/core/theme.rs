use std::ops::Index;

use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use strum::EnumCount;

use crate::core::UpdateSet;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Default to Bevy logo grey instead of Bevy website code block grey.
    app.insert_resource(ClearColor(Color::srgb(0.157, 0.157, 0.157)));

    app.configure::<(
        ConfigHandle<Theme>,
        ThemeSpriteColor,
        ThemeUiImageColor,
        ThemeTextColors,
        ThemeBackgroundColor,
        ThemeBorderColor,
    )>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
pub struct Theme {
    pub colors: ThemeColorList,
    // TODO: pub fonts: ThemeFontList,
}

impl Config for Theme {
    const PATH: &'static str = "config/theme.ron";

    const EXTENSION: &'static str = "theme.ron";

    fn apply(&self, world: &mut World) {
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
    White,
    Invisible,

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

// TODO: Make this a generic component.
#[derive(Component, Reflect, Default)]
pub struct ThemeSpriteColor(pub ThemeColor);

impl Configure for ThemeSpriteColor {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_theme_sprite_color.in_set(UpdateSet::End));
    }
}

fn apply_theme_sprite_color(
    theme_handle: Res<ConfigHandle<Theme>>,
    theme: Res<Assets<Theme>>,
    mut sprite_query: Query<(&ThemeSpriteColor, &mut Sprite)>,
) {
    let Some(palette) = &theme.get(&theme_handle.0).map(|theme| &theme.colors) else {
        return;
    };

    for (color, mut sprite) in &mut sprite_query {
        sprite.color = palette[color.0];
    }
}

#[derive(Component, Reflect, Default)]
pub struct ThemeUiImageColor(pub ThemeColor);

impl Configure for ThemeUiImageColor {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_theme_ui_image_color.in_set(UpdateSet::End));
    }
}

fn apply_theme_ui_image_color(
    theme_handle: Res<ConfigHandle<Theme>>,
    theme: Res<Assets<Theme>>,
    mut ui_image_query: Query<(&ThemeUiImageColor, &mut UiImage)>,
) {
    let Some(palette) = &theme.get(&theme_handle.0).map(|theme| &theme.colors) else {
        return;
    };

    for (color, mut image) in &mut ui_image_query {
        image.color = palette[color.0];
    }
}

#[derive(Component, Reflect, Default)]
pub struct ThemeTextColors(pub Vec<ThemeColor>);

impl Configure for ThemeTextColors {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_theme_text_colors.in_set(UpdateSet::End));
    }
}

fn apply_theme_text_colors(
    theme_handle: Res<ConfigHandle<Theme>>,
    theme: Res<Assets<Theme>>,
    mut text_query: Query<(&ThemeTextColors, &mut Text)>,
) {
    let Some(palette) = &theme.get(&theme_handle.0).map(|theme| &theme.colors) else {
        return;
    };

    for (colors, mut text) in &mut text_query {
        for (section, &color) in text.sections.iter_mut().zip(&colors.0) {
            section.style.color = palette[color];
        }
    }
}

#[derive(Component, Reflect, Default)]
pub struct ThemeBackgroundColor(pub ThemeColor);

impl Configure for ThemeBackgroundColor {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_theme_background_color.in_set(UpdateSet::End));
    }
}

fn apply_theme_background_color(
    theme_handle: Res<ConfigHandle<Theme>>,
    theme: Res<Assets<Theme>>,
    mut background_query: Query<(&ThemeBackgroundColor, &mut BackgroundColor)>,
) {
    let Some(palette) = &theme.get(&theme_handle.0).map(|theme| &theme.colors) else {
        return;
    };

    for (color, mut background) in &mut background_query {
        background.0 = palette[color.0];
    }
}

#[derive(Component, Reflect, Default)]
pub struct ThemeBorderColor(pub ThemeColor);

impl Configure for ThemeBorderColor {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Update, apply_theme_border_color.in_set(UpdateSet::End));
    }
}

fn apply_theme_border_color(
    theme_handle: Res<ConfigHandle<Theme>>,
    theme: Res<Assets<Theme>>,
    mut border_query: Query<(&ThemeBorderColor, &mut BorderColor)>,
) {
    let Some(palette) = &theme.get(&theme_handle.0).map(|theme| &theme.colors) else {
        return;
    };

    for (color, mut border) in &mut border_query {
        border.0 = palette[color.0];
    }
}
