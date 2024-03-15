// Disable common false-positive clippy warnings
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

mod common;
mod sequence;
mod ui;
mod util;

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

        app.add_plugins((
            common::CommonPlugin,
            sequence::SequencePlugin,
            ui::UiPlugin,
            util::UtilPlugin,
        ));
    }
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
