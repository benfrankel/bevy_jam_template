use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::config::Config;
use crate::state::game::GameAssets;
use crate::state::AppState;
use crate::state::AppState::*;
use crate::ui::FONT_HANDLE;
use crate::AppRoot;

pub struct TitleScreenStatePlugin;

impl Plugin for TitleScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TitleScreenAssets>()
            .init_collection::<TitleScreenAssets>()
            .add_loading_state(LoadingState::new(TitleScreen))
            .add_collection_to_loading_state::<_, GameAssets>(TitleScreen)
            .add_plugins(ProgressPlugin::new(TitleScreen))
            .init_resource::<ActionState<TitleScreenAction>>()
            .add_plugins(InputManagerPlugin::<TitleScreenAction>::default())
            .add_systems(OnEnter(TitleScreen), enter_title_screen)
            .add_systems(OnExit(TitleScreen), exit_title_screen)
            .add_systems(
                Update,
                (
                    title_screen_action_start
                        .run_if(action_just_pressed(TitleScreenAction::Start))
                        .after(TrackedProgressSet),
                    title_screen_action_quit.run_if(action_just_pressed(TitleScreenAction::Quit)),
                ),
            );
    }
}

const TITLE: &str = "The Title";

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleScreenAssets {}

#[derive(Actionlike, Reflect, Clone)]
enum TitleScreenAction {
    Start,
    Quit,
}

fn enter_title_screen(mut commands: Commands, root: Res<AppRoot>, config: Res<Config>) {
    commands.insert_resource(ClearColor(config.bg_color));

    commands.insert_resource(
        InputMap::default()
            .insert(MouseButton::Left, TitleScreenAction::Start)
            .insert(GamepadButtonType::Start, TitleScreenAction::Start)
            .insert(KeyCode::Return, TitleScreenAction::Start)
            .insert(KeyCode::Space, TitleScreenAction::Start)
            .insert(KeyCode::Escape, TitleScreenAction::Quit)
            .insert(KeyCode::Q, TitleScreenAction::Quit)
            .build(),
    );

    let screen = commands
        .spawn((
            Name::new("TitleScreen"),
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
            Name::new("Title"),
            TextBundle {
                style: Style {
                    margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(5.0), Val::Auto),
                    height: Val::Percent(8.0),
                    ..default()
                },
                text: Text::from_section(
                    TITLE,
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

fn exit_title_screen(mut commands: Commands, root: Res<AppRoot>) {
    commands.remove_resource::<InputMap<TitleScreenAction>>();
    // TODO: This and the other despawn_decendants() should probably make use of DespawnSet...
    commands.entity(root.ui).despawn_descendants();
}

fn title_screen_action_start(
    mut next_state: ResMut<NextState<AppState>>,
    progress: Res<ProgressCounter>,
) {
    // Show loading screen only if assets are still loading
    let Progress { done, total } = progress.progress_complete();
    next_state.set(if done >= total { Game } else { LoadingScreen });
}

fn title_screen_action_quit(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit);
}
