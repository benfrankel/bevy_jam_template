use bevy::audio::AudioPlugin;

use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<AudioSettings>();

    app.add_plugins(AudioPlugin::default());
}

#[derive(Resource, Reflect, Clone, Debug)]
#[reflect(Resource)]
pub struct AudioSettings {
    pub master_volume: f32,
    pub sfx_volume: f32,
    pub music_volume: f32,
    pub ui_volume: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 0.5,
            music_volume: 0.5,
            sfx_volume: 0.5,
            ui_volume: 0.5,
        }
    }
}

impl Configure for AudioSettings {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_resource::<Self>();
        app.configure::<(IsMusicAudio, IsSfxAudio, IsUiAudio)>();
        app.add_systems(
            Update,
            apply_audio_settings
                .run_if(resource_changed::<Self>)
                .in_set(UpdateSystems::Update),
        );
    }
}

impl AudioSettings {
    pub fn music_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.music_volume)
    }

    pub fn sfx_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.sfx_volume)
    }

    pub fn ui_volume(&self) -> Volume {
        Volume::Linear(self.master_volume * self.ui_volume)
    }
}

fn apply_audio_settings(
    audio_settings: Res<AudioSettings>,
    music_audio_query: Query<Entity, With<IsMusicAudio>>,
    sfx_audio_query: Query<Entity, With<IsSfxAudio>>,
    ui_audio_query: Query<Entity, With<IsUiAudio>>,
    mut volume_query: Query<(Option<&mut PlaybackSettings>, Option<&mut AudioSink>)>,
) {
    // Apply music volume.
    let volume = audio_settings.music_volume();
    for entity in &music_audio_query {
        let (playback, sink) = c!(volume_query.get_mut(entity));

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }

    // Apply SFX volume.
    let volume = audio_settings.sfx_volume();
    for entity in &sfx_audio_query {
        let (playback, sink) = c!(volume_query.get_mut(entity));

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }

    // Apply UI volume.
    let volume = audio_settings.ui_volume();
    for entity in &ui_audio_query {
        let (playback, sink) = c!(volume_query.get_mut(entity));

        if let Some(mut sink) = sink {
            sink.set_volume(volume);
        } else if let Some(mut playback) = playback {
            playback.volume = volume;
        }
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsMusicAudio;

impl Configure for IsMusicAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[allow(dead_code)]
pub fn music_audio(audio_settings: &AudioSettings, handle: Handle<AudioSource>) -> impl Bundle {
    (
        Name::new("MusicAudio"),
        IsMusicAudio,
        AudioPlayer(handle),
        PlaybackSettings::LOOP.with_volume(audio_settings.music_volume()),
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsSfxAudio;

impl Configure for IsSfxAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[allow(dead_code)]
pub fn sfx_audio(audio_settings: &AudioSettings, handle: Handle<AudioSource>) -> impl Bundle {
    (
        Name::new("SfxAudio"),
        IsSfxAudio,
        AudioPlayer(handle),
        PlaybackSettings::LOOP.with_volume(audio_settings.sfx_volume()),
    )
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
struct IsUiAudio;

impl Configure for IsUiAudio {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
    }
}

#[allow(dead_code)]
pub fn ui_audio(audio_settings: &AudioSettings, handle: Handle<AudioSource>) -> impl Bundle {
    (
        Name::new("UiSample"),
        IsUiAudio,
        AudioPlayer(handle),
        PlaybackSettings::DESPAWN
            .with_volume(audio_settings.ui_volume())
            .with_speed(thread_rng().gen_range(0.9..1.5)),
    )
}
