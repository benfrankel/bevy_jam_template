//! Game mechanics and content

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameRoot>().init_resource::<GameRoot>();

        app.add_plugins(());
    }
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GameRoot {
    pub game: Entity,
}

impl FromWorld for GameRoot {
    fn from_world(world: &mut World) -> Self {
        let game = world
            .spawn((Name::new("Game"), SpatialBundle::default()))
            .id();

        Self { game }
    }
}
