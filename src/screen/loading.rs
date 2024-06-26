use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::core::theme::ThemeBackgroundColor;
use crate::core::theme::ThemeBorderColor;
use crate::core::theme::ThemeColor;
use crate::core::theme::ThemeTextColors;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::Screen;
use crate::util::ui::FontSize;
use crate::util::ui::UiRoot;
use crate::util::ui::THICK_FONT_HANDLE;

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(Screen::Loading).load_collection::<PlayingAssets>(),
        )
        .add_plugins(ProgressPlugin::new(Screen::Loading))
        .add_systems(OnEnter(Screen::Loading), enter_loading)
        .add_systems(OnExit(Screen::Loading), exit_loading);

        app.register_type::<IsLoadingBarFill>().add_systems(
            Update,
            update_loading
                .run_if(in_state(Screen::Loading))
                .after(TrackedProgressSet),
        );
    }
}

#[derive(Component, Reflect)]
struct IsLoadingBarFill;

fn enter_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    fade_in(&mut commands);

    let screen = spawn_loading_screen(&mut commands);
    commands.entity(screen).set_parent(ui_root.body);
}

fn exit_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn spawn_loading_screen(commands: &mut Commands) -> Entity {
    let screen = commands
        .spawn((
            Name::new("LoadingScreen"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Name::new("LoadingText"),
            TextBundle {
                style: Style {
                    margin: UiRect::all(Percent(1.0)),
                    ..default()
                },
                text: Text::from_section(
                    "Loading...",
                    TextStyle {
                        font: THICK_FONT_HANDLE,
                        ..default()
                    },
                ),
                ..default()
            },
            FontSize::new(Vw(5.0)).with_step(8.0),
            ThemeTextColors(vec![ThemeColor::BodyText]),
        ))
        .set_parent(screen);

    let loading_bar = commands
        .spawn((
            Name::new("LoadingBar"),
            NodeBundle {
                style: Style {
                    width: Percent(60.0),
                    height: Percent(8.0),
                    padding: UiRect::all(VMin(1.0)),
                    border: UiRect::all(VMin(1.0)),
                    ..default()
                },
                ..default()
            },
            ThemeBorderColor(ThemeColor::BodyText),
        ))
        .set_parent(screen)
        .id();

    commands
        .spawn((
            Name::new("LoadingBarFill"),
            NodeBundle {
                style: Style {
                    width: Percent(0.0),
                    height: Percent(100.0),
                    ..default()
                },
                ..default()
            },
            ThemeBackgroundColor(ThemeColor::BodyText),
            IsLoadingBarFill,
        ))
        .set_parent(loading_bar);

    screen
}

fn update_loading(
    mut commands: Commands,
    progress: Res<ProgressCounter>,
    frame: Res<FrameCount>,
    mut loading_bar_query: Query<&mut Style, With<IsLoadingBarFill>>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to next screen when ready
    if done == total {
        fade_out(&mut commands, Screen::Playing);
    }

    // Update loading bar
    for mut style in &mut loading_bar_query {
        style.width = Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}
