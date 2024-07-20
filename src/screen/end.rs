use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;

use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::End.on_edge(exit_end, enter_end));

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
    commands.spawn_with(fade_in);
    commands.spawn_with(end_screen).set_parent(ui_root.body);

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
}

fn exit_end(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.remove_resource::<InputMap<EndScreenAction>>();
    commands.entity(ui_root.body).despawn_descendants();
}

fn end_screen(mut entity: EntityWorldMut) {
    entity
        .add(widget::column_mid)
        .insert(Name::new("EndScreen"))
        .with_children(|children| {
            children.spawn_with(end_text);
        });
}

fn end_text(mut entity: EntityWorldMut) {
    entity.insert((
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
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
    ));
}

fn restart(mut commands: Commands) {
    commands.spawn_with(fade_out(Screen::Title));
}

fn quit(mut app_exit: EventWriter<AppExit>) {
    app_exit.send(AppExit::Success);
}
