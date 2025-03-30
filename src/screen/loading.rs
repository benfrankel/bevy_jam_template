use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::FadeOut;
use crate::screen::playing::PlayingAssets;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading.bevy()).load_collection::<PlayingAssets>(),
    );
    app.add_systems(StateFlush, Screen::Loading.on_enter(loading.spawn()));

    app.configure::<IsLoadingBarFill>();
}

fn loading(In(id): In<Entity>, mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands
        .entity(id)
        .insert(Node::COLUMN_CENTER.full_size().named("Loading"))
        .set_parent(screen_root.ui)
        .with_children(|parent| {
            parent.spawn_fn(loading_text);
            parent.spawn_fn(loading_bar);
        });
}

fn loading_text(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Name::new("LoadingText"),
        RichText::from_sections(parse_rich("[t]Loading...")),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
        Node {
            margin: UiRect::all(Percent(1.0)),
            ..default()
        },
    ));
}

fn loading_bar(In(id): In<Entity>, mut commands: Commands) {
    commands
        .entity(id)
        .insert((
            Node {
                width: Percent(60.0),
                height: Percent(8.0),
                margin: UiRect::all(VMin(2.0)),
                padding: UiRect::all(VMin(1.0)),
                border: UiRect::all(VMin(1.0)),
                ..default()
            }
            .named("LoadingBar"),
            ThemeColor::BodyText.set::<BorderColor>(),
        ))
        .with_children(|parent| {
            parent.spawn_fn(loading_bar_fill);
        });
}

fn loading_bar_fill(In(id): In<Entity>, mut commands: Commands) {
    commands.entity(id).insert((
        Node::DEFAULT.full_height().named("LoadingBarFill"),
        ThemeColor::Primary.set::<BackgroundColor>(),
        IsLoadingBarFill,
    ));
}

#[derive(Component, Reflect)]
struct IsLoadingBarFill;

impl Configure for IsLoadingBarFill {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_systems(
            Update,
            Screen::Loading.on_update(
                // TODO: System ordering so this runs after all the track progress systems.
                update_loading,
            ),
        );
    }
}

fn update_loading(
    mut commands: Commands,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
    frame: Res<FrameCount>,
    mut fill_query: Query<&mut Node, With<IsLoadingBarFill>>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to next screen when ready.
    if done == total {
        commands.spawn_with(FadeOut::to(Screen::Playing));
    }

    // Update loading bar fill.
    for mut node in &mut fill_query {
        node.width = Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}
