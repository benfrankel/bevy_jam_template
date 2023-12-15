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
mod window;

use bevy::log::LogPlugin;
use bevy::prelude::*;

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
                AppSet::Tick,
                AppSet::Despawn,
                AppSet::ApplyDeferred,
                AppSet::End,
            )
                .chain(),
        )
        .add_systems(Update, apply_deferred.in_set(AppSet::ApplyDeferred));

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
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(debug::DebugPlugin {
            ambiguity_detection: false,
            //editor: false,
            ..default()
        });
    }
}

#[derive(SystemSet, Clone, Eq, PartialEq, Hash, Debug)]
pub enum AppSet {
    /// Initialize start-of-frame values
    Start,
    /// Tick timers
    Tick,
    /// Queue despawn commands
    Despawn,
    /// Apply all commands (e.g. spawn, despawn)
    ApplyDeferred,
    /// Synchronize end-of-frame values (after commands have been applied)
    End,
    // TODO: Animate sets
}

// Global entities
#[derive(Resource, Reflect)]
pub struct AppRoot {
    window: Entity,
    camera: Entity,

    // Logical entities
    ui: Entity,
    world: Entity,
}

impl Default for AppRoot {
    fn default() -> Self {
        Self {
            window: Entity::PLACEHOLDER,
            camera: Entity::PLACEHOLDER,

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
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    root.world = commands
        .spawn((Name::new("World"), SpatialBundle::default()))
        .id();
}
