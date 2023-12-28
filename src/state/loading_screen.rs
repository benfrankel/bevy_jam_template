use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::state::game::GameAssets;
use crate::state::AppState::*;
use crate::state::FADE_IN_SECS;
use crate::state::FADE_OUT_SECS;
use crate::theme::ThemeBackgroundColor;
use crate::theme::ThemeBorderColor;
use crate::theme::ThemeColor;
use crate::theme::ThemeTextColors;
use crate::ui::fade_in;
use crate::ui::fade_out;
use crate::ui::FontSize;
use crate::ui::THICK_FONT_HANDLE;
use crate::AppRoot;

pub struct LoadingScreenStatePlugin;

impl Plugin for LoadingScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(LoadingState::new(LoadingScreen))
            .add_collection_to_loading_state::<_, GameAssets>(LoadingScreen)
            .add_plugins(ProgressPlugin::new(LoadingScreen))
            .add_systems(OnEnter(LoadingScreen), enter_loading)
            .add_systems(OnExit(LoadingScreen), exit_loading);

        app.register_type::<IsLoadingBarFill>().add_systems(
            Update,
            update_loading
                .run_if(in_state(LoadingScreen))
                .after(TrackedProgressSet),
        );
    }
}

#[derive(Component, Reflect)]
struct IsLoadingBarFill;

fn enter_loading(mut commands: Commands, root: Res<AppRoot>) {
    fade_in(&mut commands, FADE_IN_SECS);

    let screen = spawn_loading_screen(&mut commands);
    commands.entity(screen).set_parent(root.ui);
}

fn exit_loading(mut commands: Commands, root: Res<AppRoot>) {
    commands.entity(root.ui).despawn_descendants();
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
            FontSize::new(Vw(5.0)),
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

    // Continue to next state when ready
    if done == total {
        fade_out(&mut commands, FADE_OUT_SECS, Game);
    }

    // Update loading bar
    for mut style in &mut loading_bar_query {
        style.width = Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}
