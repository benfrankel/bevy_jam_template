use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AssetPlugin {
        // TODO: Workaround for https://github.com/bevyengine/bevy/issues/10157
        #[cfg(feature = "web")]
        meta_check: bevy::asset::AssetMetaCheck::Never,
        ..default()
    });
}
