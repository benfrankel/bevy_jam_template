mod audio;
mod camera;
mod config;
#[cfg(feature = "dev")]
mod debug;
mod physics;
mod state;
mod ui;

use bevy::log::LogPlugin;
use bevy::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AppRoot>()
            .init_resource::<AppRoot>()
            .add_systems(Startup, spawn_logical_entities);

        // Order-dependent plugins
        app.add_plugins((
            LogPlugin::default(),
            config::ConfigPlugin,
            DefaultPlugins
                .build()
                .disable::<LogPlugin>()
                .disable::<WindowPlugin>()
                .set(ImagePlugin::default_nearest()),
            audio::AudioPlugin,
            state::StatePlugin,
        ));

        // Other plugins
        app.add_plugins((camera::CameraPlugin, physics::PhysicsPlugin, ui::UiPlugin));

        #[cfg(feature = "dev")]
        app.add_plugins(debug::DebugPlugin {
            ambiguity_detection: false,
            //editor: false,
            ..default()
        });
    }
}

#[derive(Resource, Reflect)]
pub struct AppRoot {
    camera: Entity,

    // Logical entities
    ui: Entity,
    world: Entity,
}

impl Default for AppRoot {
    fn default() -> Self {
        Self {
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
