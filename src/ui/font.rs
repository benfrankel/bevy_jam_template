use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

use crate::AppRoot;
use crate::AppSet;

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        load_internal_binary_asset!(
            app,
            FONT_HANDLE,
            "../../assets/font/PyriousPixel-R.ttf",
            |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
        );
        load_internal_binary_asset!(
            app,
            BOLD_FONT_HANDLE,
            "../../assets/font/PyriousPixel-B.ttf",
            |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
        );

        app.register_type::<FontSize>()
            .add_systems(Update, apply_font_size.in_set(AppSet::End));
    }
}

pub const FONT_HANDLE: Handle<Font> =
    Handle::weak_from_u128(303551798864246209986336759745415587961);
pub const BOLD_FONT_HANDLE: Handle<Font> =
    Handle::weak_from_u128(317423448069604009516378143395193332978);

#[derive(Component, Reflect)]
pub struct FontSize {
    size: Val,
    cache: f32,
}

impl FontSize {
    pub fn new(size: Val) -> Self {
        Self { size, cache: -1.0 }
    }
}

pub fn apply_font_size(
    root: Res<AppRoot>,
    window_query: Query<&Window>,
    mut font_size_query: Query<(&mut FontSize, &Node, &mut Text)>,
) {
    let Ok(window) = window_query.get(root.window) else {
        return;
    };
    let viewport_size = Vec2::new(window.resolution.width(), window.resolution.height());

    for (mut font_size, node, mut text) in &mut font_size_query {
        let Ok(resolved) = font_size.size.resolve(node.size().x, viewport_size) else {
            continue;
        };
        if font_size.cache == resolved {
            continue;
        }
        font_size.cache = resolved;

        for section in &mut text.sections {
            section.style.font_size = resolved;
        }
    }
}
