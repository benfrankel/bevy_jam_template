use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin as KiraAudioPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(KiraAudioPlugin);
}
