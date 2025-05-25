use std::ops::Index;

use bevy::ecs::component::Mutable;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Default to Bevy logo grey instead of Bevy website code block grey.
    app.insert_resource(ClearColor(Color::srgb(0.157, 0.157, 0.157)));

    app.configure::<(
        ConfigHandle<ThemeConfig>,
        ThemeColorFor<Sprite>,
        ThemeColorFor<ImageNode>,
        ThemeColorFor<BackgroundColor>,
        ThemeColorFor<BorderColor>,
        ThemeColorForText,
    )>();
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ThemeConfig {
    pub colors: ThemeColorList,
}

impl Config for ThemeConfig {
    const FILE: &'static str = "theme.ron";

    fn on_load(&self, world: &mut World) {
        r!(world.get_resource_mut::<ClearColor>()).0 = self.colors[ThemeColor::Body];
    }
}

// Note: The length of this array MUST equal the number of `ThemeColor` variants.
#[derive(Reflect, Serialize, Deserialize)]
pub struct ThemeColorList([Color; 11]);

impl Index<ThemeColor> for ThemeColorList {
    type Output = Color;

    fn index(&self, index: ThemeColor) -> &Self::Output {
        &self.0[index as usize]
    }
}

/// See: <https://getbootstrap.com/docs/5.3/customize/color/>.
#[derive(Reflect, Clone, Copy, Default)]
pub enum ThemeColor {
    // Absolute colors.
    #[default]
    White,
    Invisible,

    // Semantic colors.
    Body,
    BodyText,

    Primary,
    PrimaryHovered,
    PrimaryPressed,
    PrimaryDisabled,
    PrimaryText,

    // Other UI colors.
    Popup,
    Overlay,
}

impl ThemeColor {
    pub const fn set<C: ColorMut>(self) -> ThemeColorFor<C> {
        ThemeColorFor(self, PhantomData)
    }
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ThemeColorFor<C: ColorMut>(pub ThemeColor, #[reflect(ignore)] PhantomData<C>);

impl<C: ColorMut + TypePath> Configure for ThemeColorFor<C> {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_theme_color_for::<C>.in_set(UpdateSystems::SyncLate),
        );
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_theme_color_for<C: ColorMut>(
    config: ConfigRef<ThemeConfig>,
    mut color_query: Query<(&ThemeColorFor<C>, &mut C)>,
) {
    let palette = r!(config.get().map(|x| &x.colors));
    for (theme_color, mut color) in &mut color_query {
        *color.color_mut() = palette[theme_color.0];
    }
}

#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct ThemeColorForText(pub Vec<ThemeColor>);

impl Configure for ThemeColorForText {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            apply_theme_color_for_text.in_set(UpdateSystems::SyncLate),
        );
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_theme_color_for_text(
    config: ConfigRef<ThemeConfig>,
    mut text_query: Query<(&ThemeColorForText, &mut RichText)>,
) {
    let palette = r!(config.get().map(|x| &x.colors));
    for (colors, mut text) in &mut text_query {
        for (section, &color) in text.sections.iter_mut().zip(&colors.0) {
            section.style.color = palette[color];
        }
    }
}

pub trait ColorMut: Component<Mutability = Mutable> {
    fn color_mut(&mut self) -> &mut Color;
}

impl ColorMut for Sprite {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
}

impl ColorMut for ImageNode {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
}

impl ColorMut for BackgroundColor {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.0
    }
}

impl ColorMut for BorderColor {
    fn color_mut(&mut self) -> &mut Color {
        &mut self.0
    }
}
