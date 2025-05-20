use bevy::audio::AudioPlugin;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<AudioConfig>, IsAudioMusic, IsAudioUi)>();

    app.add_plugins(AudioPlugin::default());
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct AudioConfig {
    pub global_volume: f32,
    pub music_volume: f32,
    pub ui_volume: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            global_volume: 0.5,
            music_volume: 0.5,
            ui_volume: 0.5,
        }
    }
}

impl Config for AudioConfig {
    const FILE: &'static str = "audio.ron";

    fn on_load(&mut self, world: &mut World) {
        // Set global volume.
        r!(world.get_resource_mut::<GlobalVolume>()).volume = Volume::Linear(self.global_volume);

        // Update the volume of currently-playing music audio.
        for mut audio in world
            .query_filtered::<&mut AudioSink, With<IsAudioMusic>>()
            .iter_mut(world)
        {
            audio.set_volume(Volume::Linear(self.music_volume));
        }

        // Update the volume of currently-playing UI audio.
        for mut audio in world
            .query_filtered::<&mut AudioSink, With<IsAudioUi>>()
            .iter_mut(world)
        {
            audio.set_volume(Volume::Linear(self.ui_volume));
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsAudioMusic;

impl Configure for IsAudioMusic {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsAudioUi;

impl Configure for IsAudioUi {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
