// Disable common false-positive clippy warnings
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod audio;
mod camera;
mod config;
#[cfg(feature = "dev")]
mod debug;
mod physics;
mod state;
mod theme;
mod ui;
mod util;
mod window;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_mod_picking::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Global entities
        app.register_type::<AppRoot>()
            .init_resource::<AppRoot>()
            .add_systems(Startup, spawn_logical_entities);

        // Global system ordering
        app.configure_sets(
            Update,
            (
                AppSet::Start,
                AppSet::Update,
                AppSet::RecordIntents,
                AppSet::ApplyIntents,
                (AppSet::HandleEvents, AppSet::QueueCommands),
                AppSet::FlushCommands,
                AppSet::UpdateUi,
                AppSet::End,
            )
                .chain(),
        )
        .add_systems(Update, apply_deferred.in_set(AppSet::FlushCommands));

        // Work-around for https://github.com/bevyengine/bevy/issues/10157
        #[cfg(feature = "web")]
        app.insert_resource(bevy::asset::AssetMetaCheck::Never);

        // Order-dependent plugins
        app.add_plugins((
            LogPlugin::default(),
            window::WindowPlugin,
            DefaultPlugins
                .build()
                .disable::<LogPlugin>()
                .disable::<WindowPlugin>()
                .set(ImagePlugin::default_nearest()),
        ));

        // Other plugins
        app.add_plugins((
            audio::AudioPlugin,
            camera::CameraPlugin,
            config::ConfigPlugin,
            physics::PhysicsPlugin,
            state::StatePlugin,
            theme::ThemePlugin,
            ui::UiPlugin,
            util::UtilPlugin,
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(debug::DebugPlugin {
            ambiguity_detection: false,
            //editor: false,
            ..default()
        });
    }
}

/// Global system sets
#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppSet {
    /// (Update) Initialize start-of-frame values and tick timers
    Start,
    /// (Update) Step game logic
    Update,
    /// (Update) Record player and AI intents
    RecordIntents,
    /// (Update) Apply player and AI intents
    ApplyIntents,
    /// (Update) Handle events emitted this frame
    HandleEvents,
    /// (Update) Queue spawn / despawn commands
    QueueCommands,
    /// (Update) Apply spawn / despawn and other commands
    FlushCommands,
    /// (Update) Update UI
    UpdateUi,
    /// (Update) Synchronize end-of-frame values
    End,
}

// Global entities
#[derive(Resource, Reflect)]
pub struct AppRoot {
    window: Entity,
    camera: Entity,
    tooltip: Entity,
    tooltip_text: Entity,

    // Logical entities
    ui: Entity,
    world: Entity,
}

impl Default for AppRoot {
    fn default() -> Self {
        Self {
            window: Entity::PLACEHOLDER,
            camera: Entity::PLACEHOLDER,
            tooltip: Entity::PLACEHOLDER,
            tooltip_text: Entity::PLACEHOLDER,

            ui: Entity::PLACEHOLDER,
            world: Entity::PLACEHOLDER,
        }
    }
}

fn spawn_logical_entities(mut commands: Commands, mut root: ResMut<AppRoot>) {
    root.ui = commands
        .spawn((
            Name::new("Ui"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                ..default()
            },
            Pickable::IGNORE,
        ))
        .id();

    root.world = commands
        .spawn((Name::new("World"), SpatialBundle::default()))
        .id();
}
