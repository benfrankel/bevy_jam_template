use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::state::fade_in;
use crate::state::fade_out;
use crate::state::AppState::*;
use crate::theme::ThemeColor;
use crate::theme::ThemeTextColors;
use crate::ui::FontSize;
use crate::ui::BOLD_FONT_HANDLE;
use crate::AppRoot;

pub struct EndScreenStatePlugin;

impl Plugin for EndScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EndScreenAssets>()
            .init_collection::<EndScreenAssets>();

        app.init_resource::<ActionState<EndScreenAction>>()
            .add_plugins(InputManagerPlugin::<EndScreenAction>::default())
            .add_systems(
                Update,
                (
                    restart.run_if(action_just_pressed(EndScreenAction::Restart)),
                    quit.run_if(action_just_pressed(EndScreenAction::Quit)),
                ),
            );

        app.add_systems(OnEnter(EndScreen), enter_end_screen)
            .add_systems(OnExit(EndScreen), exit_end_screen);
    }
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct EndScreenAssets {}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
enum EndScreenAction {
    Restart,
    Quit,
}

fn enter_end_screen(mut commands: Commands, root: Res<AppRoot>) {
    fade_in(&mut commands);

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

    let screen = spawn_end_screen(&mut commands);
    commands.entity(screen).set_parent(root.ui);
}

fn exit_end_screen(mut commands: Commands, root: Res<AppRoot>) {
    commands.remove_resource::<InputMap<EndScreenAction>>();
    commands.entity(root.ui).despawn_descendants();
}

fn spawn_end_screen(commands: &mut Commands) -> Entity {
    let screen = commands
        .spawn((
            Name::new("EndScreen"),
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
            Name::new("EndText"),
            TextBundle {
                style: Style {
                    margin: UiRect::top(Percent(5.0)),
                    height: Percent(8.0),
                    ..default()
                },
                text: Text::from_section(
                    "The End",
                    TextStyle {
                        font: BOLD_FONT_HANDLE,
                        ..default()
                    },
                ),
                ..default()
            },
            FontSize::new(Vw(5.0)).with_step(8.0),
            ThemeTextColors(vec![ThemeColor::BodyText]),
        ))
        .set_parent(screen);

    screen
}

fn restart(mut commands: Commands) {
    fade_out(&mut commands, TitleScreen);
}

fn quit(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit);
}
