use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::playing::PlayingAssets;
use crate::screen::Screen;
use crate::ui::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(LoadingState::new(Screen::Loading).load_collection::<PlayingAssets>());
    app.add_plugins(ProgressPlugin::new(Screen::Loading));
    app.add_systems(OnEnter(Screen::Loading), enter_loading);
    app.add_systems(OnExit(Screen::Loading), exit_loading);

    app.register_type::<IsLoadingBarFill>();
    app.add_systems(
        Update,
        update_loading
            .run_if(in_state(Screen::Loading))
            .after(TrackedProgressSet),
    );
}

#[derive(Component, Reflect)]
struct IsLoadingBarFill;

fn enter_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(fade_in);
    commands.spawn_with(loading_screen).set_parent(ui_root.body);
}

fn exit_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn loading_screen(mut entity: EntityWorldMut) {
    entity
        .add(widget::ui_root)
        .insert(Name::new("LoadingScreen"))
        .with_children(|children| {
            children.spawn_with(loading_text);
            children.spawn_with(loading_bar);
        });
}

fn loading_text(mut entity: EntityWorldMut) {
    entity.insert((
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
    ));
}

fn loading_bar(mut entity: EntityWorldMut) {
    entity
        .insert((
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
        .with_children(|children| {
            children.spawn_with(loading_bar_fill);
        });
}

fn loading_bar_fill(mut entity: EntityWorldMut) {
    entity.insert((
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
    ));
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
        commands.spawn_with(fade_out(Screen::Playing));
    }

    // Update loading bar
    for mut style in &mut loading_bar_query {
        style.width = Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}
