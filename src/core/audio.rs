use bevy::audio::AudioPlugin;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(ConfigHandle<AudioConfig>, IsMusicAudio, IsUiAudio)>();

    app.add_plugins(AudioPlugin::default());
}

#[derive(Asset, Reflect, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct AudioConfig {
    pub master_volume: f32,
    pub music_volume: f32,
    pub ui_volume: f32,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            master_volume: 0.5,
            music_volume: 0.5,
            ui_volume: 0.5,
        }
    }
}

impl Config for AudioConfig {
    const FILE: &'static str = "audio.ron";

    fn on_load(&self, world: &mut World) {
        // Update the volume of currently-playing audio by type.
        world
            .query_filtered::<&mut AudioSink, With<IsMusicAudio>>()
            .iter_mut(world)
            .for_each(|mut sink| sink.set_volume(self.music_volume()));
        world
            .query_filtered::<&mut AudioSink, With<IsUiAudio>>()
            .iter_mut(world)
            .for_each(|mut sink| sink.set_volume(self.ui_volume()));
    }
}

impl AudioConfig {
    pub fn music_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.music_volume)
    }

    pub fn ui_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.ui_volume)
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsMusicAudio;

impl Configure for IsMusicAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct IsUiAudio;

impl Configure for IsUiAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}
