use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin as KiraAudioPlugin;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(KiraAudioPlugin);
    }
}
