use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::config::Config;
use crate::state::AppState;
use crate::state::AppState::*;
use crate::ui::FONT_HANDLE;
use crate::AppRoot;

pub struct EndScreenStatePlugin;

impl Plugin for EndScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EndScreenAssets>()
            .init_collection::<EndScreenAssets>()
            .init_resource::<ActionState<EndScreenAction>>()
            .add_plugins(InputManagerPlugin::<EndScreenAction>::default())
            .add_systems(OnEnter(EndScreen), enter_end_screen)
            .add_systems(OnExit(EndScreen), exit_end_screen)
            .add_systems(
                Update,
                (
                    end_screen_action_restart.run_if(action_just_pressed(EndScreenAction::Restart)),
                    end_screen_action_quit.run_if(action_just_pressed(EndScreenAction::Quit)),
                ),
            );
    }
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct EndScreenAssets {}

#[derive(Actionlike, Reflect, Clone)]
enum EndScreenAction {
    Restart,
    Quit,
}

fn enter_end_screen(mut commands: Commands, root: Res<AppRoot>, config: Res<Config>) {
    commands.insert_resource(ClearColor(config.bg_color));

    commands.insert_resource(
        InputMap::default()
            .insert(MouseButton::Left, EndScreenAction::Restart)
            .insert(GamepadButtonType::Start, EndScreenAction::Restart)
            .insert(KeyCode::Return, EndScreenAction::Restart)
            .insert(KeyCode::Space, EndScreenAction::Restart)
            .insert(KeyCode::Escape, EndScreenAction::Quit)
            .insert(KeyCode::Q, EndScreenAction::Quit)
            .build(),
    );

    let screen = commands
        .spawn((
            Name::new("EndScreen"),
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .set_parent(root.ui)
        .id();

    commands
        .spawn((
            Name::new("TheEnd"),
            TextBundle {
                style: Style {
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(5.0), Val::Auto),
                    height: Val::Percent(8.0),
                    ..default()
                },
                text: Text::from_section(
                    "The End",
                    TextStyle {
                        font: FONT_HANDLE,
                        font_size: 64.0,
                        color: config.fg_color,
                    },
                ),
                ..default()
            },
        ))
        .set_parent(screen);
}

fn exit_end_screen(mut commands: Commands, root: Res<AppRoot>) {
    commands.remove_resource::<InputMap<EndScreenAction>>();
    commands.entity(root.ui).despawn_descendants();
}

fn end_screen_action_restart(mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(TitleScreen);
}

fn end_screen_action_quit(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit);
}
