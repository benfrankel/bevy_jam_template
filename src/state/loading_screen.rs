use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::state::game::GameAssets;
use crate::state::AppState::*;
use crate::theme::PaletteColor;
use crate::ui::FontSize;
use crate::ui::FONT_HANDLE;
use crate::AppRoot;

pub struct LoadingScreenStatePlugin;

impl Plugin for LoadingScreenStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IsLoadingBarFill>()
            .add_loading_state(LoadingState::new(LoadingScreen))
            .add_collection_to_loading_state::<_, GameAssets>(LoadingScreen)
            .add_plugins(ProgressPlugin::new(LoadingScreen).continue_to(Game))
            .add_systems(OnEnter(LoadingScreen), enter_loading)
            .add_systems(OnExit(LoadingScreen), exit_loading)
            .add_systems(
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
    let screen = commands
        .spawn((
            Name::new("LoadingScreen"),
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
            Name::new("LoadingContainer"),
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .set_parent(screen)
        .with_children(|commands| {
            commands.spawn((
                Name::new("LoadingHeader"),
                TextBundle {
                    style: Style {
                        margin: UiRect::all(Val::Percent(1.0)),
                        ..default()
                    },
                    text: Text::from_section(
                        "Loading...",
                        TextStyle {
                            font: FONT_HANDLE,
                            ..default()
                        },
                    ),
                    ..default()
                },
                FontSize::new(Val::Vw(5.0)),
                PaletteColor::Foreground,
            ));

            commands
                .spawn((
                    Name::new("LoadingBarContainer"),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(50.0),
                            height: Val::Percent(5.0),
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|commands| {
                    commands.spawn((
                        Name::new("LoadingBarFill"),
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(0.0),
                                height: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        // TODO: BackgroundPaletteColor
                        PaletteColor::Foreground,
                        IsLoadingBarFill,
                    ));
                });
        });
}

fn exit_loading(mut commands: Commands, root: Res<AppRoot>) {
    commands.entity(root.ui).despawn_descendants();
}

fn update_loading(
    mut loading_bar_query: Query<&mut Style, With<IsLoadingBarFill>>,
    progress: Res<ProgressCounter>,
    frame: Res<FrameCount>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    for mut style in &mut loading_bar_query {
        style.width = Val::Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}
