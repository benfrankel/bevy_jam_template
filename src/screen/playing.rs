use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::core::camera::CameraRoot;
use crate::core::UpdateSet;
use crate::screen::fade_in;
use crate::screen::Screen;
use crate::util::ui::UiRoot;

pub struct PlayingScreenPlugin;

impl Plugin for PlayingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameAssets>()
            .init_collection::<GameAssets>();

        app.add_systems(OnEnter(Screen::Playing), enter_game)
            .add_systems(OnExit(Screen::Playing), exit_game)
            .add_systems(
                OnEnter(Screen::PlayingRestart),
                |mut screen: ResMut<NextState<_>>| {
                    screen.set(Screen::Playing);
                },
            );

        app.init_resource::<ActionState<GameAction>>()
            .insert_resource(
                InputMap::default()
                    .insert(GameAction::Restart, KeyCode::KeyR)
                    .build(),
            )
            .add_plugins(InputManagerPlugin::<GameAction>::default())
            .add_systems(
                Update,
                restart.in_set(UpdateSet::HandleActions).run_if(
                    in_state(Screen::Playing).and_then(action_just_pressed(GameAction::Restart)),
                ),
            );
    }
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameAssets {}

fn enter_game(mut commands: Commands) {
    fade_in(&mut commands);
}

fn exit_game(
    mut commands: Commands,
    ui_root: Res<UiRoot>,
    camera_root: Res<CameraRoot>,
    mut camera_query: Query<&mut Transform>,
) {
    // Reset resources

    // Clear events

    // Despawn entities
    commands.entity(ui_root.body).despawn_descendants();

    // Reset camera
    if let Ok(mut transform) = camera_query.get_mut(camera_root.primary) {
        transform.translation = Vec2::ZERO.extend(transform.translation.z);
    };
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
pub enum GameAction {
    Restart,
    // TODO: Pause
}

fn restart(mut screen: ResMut<NextState<Screen>>) {
    screen.set(Screen::PlayingRestart);
}
