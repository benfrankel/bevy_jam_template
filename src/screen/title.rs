use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::*;
use iyes_progress::prelude::*;

use crate::core::theme::ThemeBackgroundColor;
use crate::core::theme::ThemeColor;
use crate::core::theme::ThemeTextColors;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::Screen;
use crate::util::ui::FontSize;
use crate::util::ui::InteractionPalette;
use crate::util::ui::UiRoot;
use crate::util::ui::BOLD_FONT_HANDLE;
use crate::util::ui::FONT_HANDLE;

pub struct TitleScreenPlugin;

impl Plugin for TitleScreenPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TitleScreenAssets>()
            .init_collection::<TitleScreenAssets>();

        app.add_loading_state(LoadingState::new(Screen::Title).load_collection::<PlayingAssets>())
            .add_plugins(ProgressPlugin::new(Screen::Title))
            .add_systems(OnEnter(Screen::Title), enter_title)
            .add_systems(OnExit(Screen::Title), exit_title);
    }
}

const TITLE: &str = "bevy_jam_template";

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleScreenAssets {}

fn enter_title(mut commands: Commands, ui_root: Res<UiRoot>) {
    fade_in(&mut commands);

    let screen = spawn_title_screen(&mut commands);
    commands.entity(screen).set_parent(ui_root.body);
}

fn exit_title(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn spawn_title_screen(commands: &mut Commands) -> Entity {
    let screen = commands
        .spawn((
            Name::new("TitleScreen"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Name::new("Title"),
            TextBundle::from_section(
                TITLE,
                TextStyle {
                    font: BOLD_FONT_HANDLE,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::vertical(Vw(5.0)),
                ..default()
            }),
            FontSize::new(Vw(5.0)).with_step(8.0),
            ThemeTextColors(vec![ThemeColor::BodyText]),
        ))
        .set_parent(screen);

    let button_container = commands
        .spawn((
            Name::new("ButtonContainer"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Vw(40.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Vw(2.5),
                    ..default()
                },
                ..default()
            },
        ))
        .set_parent(screen)
        .id();

    let play_button = spawn_button(commands, "Play");
    commands
        .entity(play_button)
        .insert(On::<Pointer<Click>>::run(
            |mut commands: Commands, progress: Res<ProgressCounter>| {
                let Progress { done, total } = progress.progress_complete();
                fade_out(
                    &mut commands,
                    if done >= total {
                        Screen::Playing
                    } else {
                        Screen::Loading
                    },
                );
            },
        ))
        .set_parent(button_container);

    let quit_button = spawn_button(commands, "Quit");
    commands
        .entity(quit_button)
        .insert((
            #[cfg(feature = "web")]
            crate::util::ui::IsDisabled(true),
            #[cfg(not(feature = "web"))]
            On::<Pointer<Click>>::run(|mut app_exit: EventWriter<_>| {
                app_exit.send(bevy::app::AppExit::Success);
            }),
        ))
        .set_parent(button_container);

    screen
}

fn spawn_button(commands: &mut Commands, text: impl Into<String>) -> Entity {
    let text = text.into();

    let button = commands
        .spawn((
            Name::new(format!("{}Button", text.replace(' ', ""))),
            ButtonBundle {
                style: Style {
                    height: Vw(8.0),
                    width: Vw(30.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            ThemeBackgroundColor(ThemeColor::None),
            InteractionPalette {
                normal: ThemeColor::Primary,
                hovered: ThemeColor::PrimaryHovered,
                pressed: ThemeColor::PrimaryPressed,
                disabled: ThemeColor::PrimaryDisabled,
            },
        ))
        .id();

    commands
        .spawn((
            Name::new("ButtonText"),
            TextBundle::from_section(
                text,
                TextStyle {
                    font: FONT_HANDLE,
                    ..default()
                },
            ),
            FontSize::new(Vw(4.0)).with_step(8.0),
            ThemeTextColors(vec![ThemeColor::PrimaryText]),
        ))
        .set_parent(button);

    button
}
