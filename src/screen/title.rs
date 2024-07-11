use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::prelude::*;
use iyes_progress::prelude::*;

use crate::core::theme::ThemeColor;
use crate::core::theme::ThemeTextColors;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(Screen::Title).load_collection::<PlayingAssets>());
    app.add_plugins(ProgressPlugin::new(Screen::Title));
    app.add_systems(OnEnter(Screen::Title), enter_title);
    app.add_systems(OnExit(Screen::Title), exit_title);

    app.configure::<TitleScreenAssets>();
}

const TITLE: &str = "bevy_jam_template";

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct TitleScreenAssets {}

impl Configure for TitleScreenAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

fn enter_title(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);

    let screen = spawn_title_screen(&mut commands);
    commands.entity(screen).set_parent(ui_root.body);
}

fn exit_title(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn spawn_title_screen(commands: &mut Commands) -> Entity {
    let screen = commands
        .spawn_with(ui_root)
        .insert(Name::new("TitleScreen"))
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

    // Spawn play button.
    commands
        .spawn_with(menu_button("Play"))
        .insert(On::<Pointer<Click>>::run(
            |mut commands: Commands, progress: Res<ProgressCounter>| {
                let Progress { done, total } = progress.progress_complete();
                commands.spawn_with(fade_out(if done >= total {
                    Screen::Playing
                } else {
                    Screen::Loading
                }));
            },
        ))
        .set_parent(button_container);

    // Spawn quit button.
    commands
        .spawn_with(menu_button("Quit"))
        .insert((
            #[cfg(feature = "web")]
            IsDisabled(true),
            #[cfg(not(feature = "web"))]
            On::<Pointer<Click>>::run(|mut app_exit: EventWriter<_>| {
                app_exit.send(bevy::app::AppExit::Success);
            }),
        ))
        .set_parent(button_container);

    screen
}
