use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.configure::<AudioSettings>();

    app.add_plugins(SeedlingPlugin::default());
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
        app.configure::<(MusicPool, UiPool)>();
        app.add_systems(
            Update,
            apply_audio_settings
                .run_if(resource_changed::<Self>)
                .in_set(UpdateSystems::Update),
        );
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn apply_audio_settings(
    audio_settings: Res<AudioSettings>,
    master_query: Query<Entity, (With<MainBus>, With<VolumeNode>)>,
    music_query: Query<Entity, (With<SamplerPool<MusicPool>>, With<VolumeNode>)>,
    ui_query: Query<Entity, (With<SamplerPool<UiPool>>, With<VolumeNode>)>,
    mut volume_query: Query<&mut VolumeNode>,
) {
    // Update master volume.
    for entity in &master_query {
        c!(volume_query.get_mut(entity)).volume = Volume::Linear(audio_settings.master_volume);
    }

    // Update music volume.
    for entity in &music_query {
        c!(volume_query.get_mut(entity)).volume = Volume::Linear(audio_settings.music_volume);
    }

    // Update UI volume.
    for entity in &ui_query {
        c!(volume_query.get_mut(entity)).volume = Volume::Linear(audio_settings.ui_volume);
    }
}

#[derive(PoolLabel, Reflect, Eq, PartialEq, Hash, Clone, Debug)]
#[reflect(Component)]
struct MusicPool;

impl Configure for MusicPool {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Startup, spawn_music_pool);
    }
}

fn spawn_music_pool(mut commands: Commands) {
    commands.spawn((Name::new("MusicPool"), SamplerPool(MusicPool)));
}

pub fn music_sample(handle: Handle<Sample>) -> impl Bundle {
    (
        Name::new("MusicSample"),
        SamplePlayer::new(handle),
        PlaybackSettings::LOOP,
        MusicPool,
    )
}

#[derive(PoolLabel, Reflect, Eq, PartialEq, Hash, Clone, Debug)]
#[reflect(Component)]
struct UiPool;

impl Configure for UiPool {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(Startup, spawn_ui_pool);
    }
}

fn spawn_ui_pool(mut commands: Commands) {
    commands.spawn((Name::new("UiPool"), SamplerPool(UiPool)));
}

pub fn ui_sample(handle: Handle<Sample>) -> impl Bundle {
    (
        Name::new("UiSample"),
        SamplePlayer::new(handle),
        PlaybackSettings::ONCE,
        // TODO: Set up pitch randomization again via `bevy_seedling/rand` feature.
        UiPool,
    )
}
