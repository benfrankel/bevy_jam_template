use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::*;
use iyes_progress::prelude::*;

use crate::state::game::GameAssets;
use crate::state::AppState::*;
use crate::theme::ThemeColor;
use crate::ui::FontSize;
use crate::ui::InteractionPalette;
use crate::ui::BOLD_FONT_HANDLE;
use crate::ui::FONT_HANDLE;
use crate::AppRoot;

pub struct TitleScreenStatePlugin;

impl Plugin for TitleScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TitleScreenAssets>()
            .init_collection::<TitleScreenAssets>();

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

fn enter_title_screen(mut commands: Commands, root: Res<AppRoot>) {
    let screen = spawn_title_screen(&mut commands);
    commands.entity(screen).set_parent(root.ui);
}

fn exit_title_screen(mut commands: Commands, root: Res<AppRoot>) {
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
            FontSize::new(Vw(5.0)),
            ThemeColor::BodyText,
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
            |mut next_state: ResMut<NextState<_>>, progress: Res<ProgressCounter>| {
                let Progress { done, total } = progress.progress_complete();
                next_state.set(if done >= total { Game } else { LoadingScreen });
            },
        ))
        .set_parent(button_container);

    let quit_button = spawn_button(commands, "Quit");
    commands
        .entity(quit_button)
        .insert((
            #[cfg(feature = "web")]
            crate::ui::IsDisabled(true),
            #[cfg(not(feature = "web"))]
            On::<Pointer<Click>>::run(|mut app_exit: EventWriter<_>| {
                app_exit.send(bevy::app::AppExit);
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
            ThemeColor::None,
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
            FontSize::new(Vw(4.0)),
            ThemeColor::PrimaryText,
        ))
        .set_parent(button);

    button
}
