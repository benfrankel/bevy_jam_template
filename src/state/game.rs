use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::state::fade_in;
use crate::state::AppState::*;
use crate::AppRoot;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameAssets>()
            .init_collection::<GameAssets>();

        app.add_systems(OnEnter(Game), enter_game)
            .add_systems(OnExit(Game), exit_game);

        app.init_resource::<ActionState<GameAction>>()
            .add_plugins(InputManagerPlugin::<GameAction>::default())
            .add_systems(
                PreUpdate,
                restart.run_if(action_just_pressed(GameAction::Restart)),
            );
    }
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameAssets {}

fn enter_game(mut commands: Commands) {
    enter_game_helper(&mut commands);
}

fn enter_game_helper(commands: &mut Commands) {
    fade_in(commands);

    // Set up keybinds
    commands.insert_resource(
        InputMap::default()
            .insert(KeyCode::R, GameAction::Restart)
            .build(),
    );
}

fn exit_game(
    mut commands: Commands,
    root: Res<AppRoot>,
    mut transform_query: Query<&mut Transform>,
) {
    exit_game_helper(&mut commands, &root, &mut transform_query);
}

fn exit_game_helper(
    commands: &mut Commands,
    root: &AppRoot,
    camera_query: &mut Query<&mut Transform>,
) {
    // Remove resources
    commands.remove_resource::<InputMap<GameAction>>();

    // NOTE: Clear events

    // Despawn entities
    commands.entity(root.ui).despawn_descendants();

    // Reset camera
    if let Ok(mut transform) = camera_query.get_mut(root.camera) {
        transform.translation = Vec2::ZERO.extend(transform.translation.z);
    };
}

fn restart(mut commands: Commands, root: Res<AppRoot>, mut camera_query: Query<&mut Transform>) {
    exit_game_helper(&mut commands, &root, &mut camera_query);
    enter_game_helper(&mut commands);
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
pub enum GameAction {
    Restart,
    // TODO: Pause
}
