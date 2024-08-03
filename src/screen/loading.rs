use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::playing::PlayingAssets;
use crate::screen::FadeIn;
use crate::screen::FadeOut;
use crate::screen::Screen;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading.bevy()).load_collection::<PlayingAssets>(),
    );
    app.add_plugins(ProgressPlugin::new(Screen::Loading.bevy()));
    app.add_systems(
        StateFlush,
        Screen::Loading.on_edge(exit_loading, enter_loading),
    );

    app.register_type::<IsLoadingBarFill>();
    app.add_systems(
        Update,
        Screen::Loading.on_update(update_loading.after(TrackedProgressSet)),
    );
}

#[derive(Component, Reflect)]
struct IsLoadingBarFill;

fn enter_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.spawn_with(FadeIn::default());
    commands.spawn_fn(loading_screen).set_parent(ui_root.body);
}

fn exit_loading(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn loading_screen(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert(Style::COLUMN_CENTER.full_size().node("LoadingScreen"))
        .with_children(|children| {
            children.spawn_fn(loading_text);
            children.spawn_fn(loading_bar);
        });
}

fn loading_text(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
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
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
    ));
}

fn loading_bar(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert((
            Name::new("LoadingBar"),
            NodeBundle {
                style: Style {
                    width: Percent(60.0),
                    height: Percent(8.0),
                    margin: UiRect::all(VMin(2.0)),
                    padding: UiRect::all(VMin(1.0)),
                    border: UiRect::all(VMin(1.0)),
                    ..default()
                },
                ..default()
            },
            ThemeColor::BodyText.set::<BorderColor>(),
        ))
        .with_children(|children| {
            children.spawn_fn(loading_bar_fill);
        });
}

fn loading_bar_fill(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Name::new("LoadingBarFill"),
        NodeBundle {
            style: Style {
                width: Percent(0.0),
                height: Percent(100.0),
                ..default()
            },
            ..default()
        },
        ThemeColor::Primary.set::<BackgroundColor>(),
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
        commands.spawn_with(FadeOut::to(Screen::Playing));
    }

    // Update loading bar
    for mut style in &mut loading_bar_query {
        style.width = Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}
