use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

pub struct FontPlugin;

impl Plugin for FontPlugin {
    fn build(&self, app: &mut App) {
        load_internal_binary_asset!(
            app,
            FONT_HANDLE,
            "../../assets/font/DungeonFont.ttf",
            |bytes: &[u8], _path: String| Font::try_from_bytes(bytes.to_vec()).unwrap()
        );
    }
}

pub const FONT_HANDLE: Handle<Font> =
    Handle::weak_from_u128(317423448069604009516378143395193332978);
