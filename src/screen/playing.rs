use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::core::camera::CameraRoot;
use crate::core::UpdateSet;
use crate::screen::fade_in;
use crate::screen::Screen;
use crate::util::prelude::*;
use crate::util::ui::UiRoot;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), enter_playing);
    app.add_systems(OnExit(Screen::Playing), exit_playing);
    app.add_systems(
        OnEnter(Screen::PlayingRestart),
        |mut screen: ResMut<NextState<_>>| {
            screen.set(Screen::Playing);
        },
    );

    app.configure::<(PlayingAssets, PlayingAction)>();
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayingAssets {}

impl Configure for PlayingAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

fn enter_playing(mut commands: Commands) {
    fade_in(&mut commands);
}

fn exit_playing(
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
pub enum PlayingAction {
    Restart,
    // TODO: Pause
}

impl Configure for PlayingAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .insert(Self::Restart, KeyCode::KeyR)
                .build(),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            restart
                .in_set(UpdateSet::HandleActions)
                .run_if(in_state(Screen::Playing).and_then(action_just_pressed(Self::Restart))),
        );
    }
}

fn restart(mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::PlayingRestart);
}
