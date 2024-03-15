use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::common::camera::CameraRoot;
use crate::sequence::fade_in;
use crate::sequence::SequenceState::*;
use crate::util::ui::UiRoot;

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
            .insert(GameAction::Restart, KeyCode::KeyR)
            .build(),
    );
}

fn exit_game(
    mut commands: Commands,
    ui_root: Res<UiRoot>,
    camera_root: Res<CameraRoot>,
    mut transform_query: Query<&mut Transform>,
) {
    exit_game_helper(&mut commands, &ui_root, &camera_root, &mut transform_query);
}

fn exit_game_helper(
    commands: &mut Commands,
    ui_root: &UiRoot,
    camera_root: &CameraRoot,
    camera_query: &mut Query<&mut Transform>,
) {
    // Remove resources
    commands.remove_resource::<InputMap<GameAction>>();

    // NOTE: Clear events

    // Despawn entities
    commands.entity(ui_root.body).despawn_descendants();

    // Reset camera
    if let Ok(mut transform) = camera_query.get_mut(camera_root.primary) {
        transform.translation = Vec2::ZERO.extend(transform.translation.z);
    };
}

fn restart(
    mut commands: Commands,
    ui_root: Res<UiRoot>,
    camera_root: Res<CameraRoot>,
    mut camera_query: Query<&mut Transform>,
) {
    exit_game_helper(&mut commands, &ui_root, &camera_root, &mut camera_query);
    enter_game_helper(&mut commands);
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
pub enum GameAction {
    Restart,
    // TODO: Pause
}
