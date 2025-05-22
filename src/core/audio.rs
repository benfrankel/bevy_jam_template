use bevy::audio::AudioPlugin;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<(IsMusicAudio, IsUiAudio)>();

    app.add_plugins(AudioPlugin::default());
}

#[derive(Resource, Reflect, Clone, Debug)]
#[reflect(Resource)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub music_volume: f32,
    pub ui_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.5,
            music_volume: 0.5,
            ui_volume: 0.5,
        }
    }
}

impl Configure for AudioSettings {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.add_systems(
            Update,
            apply_audio_settings
                .run_if(resource_changed::<AudioSettings>)
                .in_set(UpdateSystems::Update),
        );
    }
}

impl AudioSettings {
    pub fn music_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.music_volume)
    }

    pub fn ui_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.ui_volume)
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_audio_settings(
    audio_settings: Res<AudioSettings>,
    music_audio_query: Query<Entity, With<IsMusicAudio>>,
    ui_audio_query: Query<Entity, With<IsUiAudio>>,
    mut sink_query: Query<&mut AudioSink>,
) {
    for entity in &music_audio_query {
        let mut sink = c!(sink_query.get_mut(entity));
        sink.set_volume(audio_settings.music_volume());
    }
    for entity in &ui_audio_query {
        let mut sink = c!(sink_query.get_mut(entity));
        sink.set_volume(audio_settings.ui_volume());
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
