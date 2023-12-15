use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::state::game::GameAssets;
use crate::state::AppState;
use crate::state::AppState::*;
use crate::theme::PaletteColor;
use crate::ui::FontSize;
use crate::ui::BOLD_FONT_HANDLE;
use crate::AppRoot;

pub struct TitleScreenStatePlugin;

impl Plugin for TitleScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TitleScreenAssets>()
            .init_collection::<TitleScreenAssets>();

        app.init_resource::<ActionState<TitleScreenAction>>()
            .add_plugins(InputManagerPlugin::<TitleScreenAction>::default())
            .add_systems(
                Update,
                (
                    start
                        .run_if(action_just_pressed(TitleScreenAction::Start))
                        .after(TrackedProgressSet),
                    quit.run_if(action_just_pressed(TitleScreenAction::Quit)),
                ),
            );

        app.add_loading_state(LoadingState::new(TitleScreen))
            .add_collection_to_loading_state::<_, GameAssets>(TitleScreen)
            .add_plugins(ProgressPlugin::new(TitleScreen))
            .add_systems(OnEnter(TitleScreen), enter_title_screen)
            .add_systems(OnExit(TitleScreen), exit_title_screen);
    }
}

const TITLE: &str = "bevy_jam_template";

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleScreenAssets {}

#[derive(Actionlike, Reflect, Clone)]
enum TitleScreenAction {
    Start,
    Quit,
}

fn enter_title_screen(mut commands: Commands, root: Res<AppRoot>) {
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

    let screen = spawn_title_screen(&mut commands);
    commands.entity(screen).set_parent(root.ui);
}

fn exit_title_screen(mut commands: Commands, root: Res<AppRoot>) {
    commands.remove_resource::<InputMap<TitleScreenAction>>();
    commands.entity(root.ui).despawn_descendants();
}

fn spawn_title_screen(commands: &mut Commands) -> Entity {
    let screen = commands
        .spawn((
            Name::new("TitleScreen"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Name::new("Title"),
            TextBundle {
                style: Style {
                    margin: UiRect::new(Auto, Auto, Percent(5.0), Auto),
                    height: Percent(8.0),
                    ..default()
                },
                text: Text::from_section(
                    TITLE,
                    TextStyle {
                        font: BOLD_FONT_HANDLE,
                        ..default()
                    },
                ),
                ..default()
            },
            FontSize::new(Vw(5.0)),
            PaletteColor::Foreground,
        ))
        .set_parent(screen);

    screen
}

fn start(mut next_state: ResMut<NextState<AppState>>, progress: Res<ProgressCounter>) {
    // Show loading screen only if assets are still loading
    let Progress { done, total } = progress.progress_complete();
    next_state.set(if done >= total { Game } else { LoadingScreen });
}

fn quit(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit);
}
