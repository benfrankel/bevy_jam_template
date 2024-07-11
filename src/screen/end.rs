use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;

use crate::core::theme::ThemeColor;
use crate::core::theme::ThemeTextColors;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::End), enter_end);
    app.add_systems(OnExit(Screen::End), exit_end);

    app.configure::<(EndScreenAssets, EndScreenAction)>();
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct EndScreenAssets {}

impl Configure for EndScreenAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
enum EndScreenAction {
    Restart,
    Quit,
}

impl Configure for EndScreenAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            (
                restart.run_if(action_just_pressed(Self::Restart)),
                quit.run_if(action_just_pressed(Self::Quit)),
            ),
        );
    }
}

fn enter_end(mut commands: Commands, ui_root: Res<UiRoot>) {
    fade_in(&mut commands);

    commands.insert_resource(
        InputMap::default()
            .insert(EndScreenAction::Restart, MouseButton::Left)
            .insert(EndScreenAction::Restart, GamepadButtonType::Start)
            .insert(EndScreenAction::Restart, KeyCode::Enter)
            .insert(EndScreenAction::Restart, KeyCode::Space)
            .insert(EndScreenAction::Quit, KeyCode::Escape)
            .insert(EndScreenAction::Quit, KeyCode::KeyQ)
            .build(),
    );

    let screen = spawn_end_screen(&mut commands);
    commands.entity(screen).set_parent(ui_root.body);
}

fn exit_end(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.remove_resource::<InputMap<EndScreenAction>>();
    commands.entity(ui_root.body).despawn_descendants();
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
    fade_out(&mut commands, Screen::Title);
}

fn quit(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
