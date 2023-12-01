use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::config::Config;
use crate::state::AppState::*;
use crate::AppRoot;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameAssets>()
            .init_collection::<GameAssets>()
            .add_systems(OnEnter(Game), enter_game)
            .add_systems(OnExit(Game), exit_game);
    }
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameAssets {}

fn enter_game(mut commands: Commands, config: Res<Config>) {
    commands.insert_resource(ClearColor(config.bg_color));
}

fn exit_game(root: Res<AppRoot>, mut transform_query: Query<&mut Transform>) {
    let Ok(mut transform) = transform_query.get_mut(root.camera) else {
        return;
    };
    transform.translation = Vec2::ZERO.extend(transform.translation.z);
}
