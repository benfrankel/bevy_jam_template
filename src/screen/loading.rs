use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::fade_out;
use crate::screen::gameplay::GameplayAssets;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Loading.bevy()).load_collection::<GameplayAssets>(),
    );
    app.add_systems(StateFlush, Screen::Loading.on_enter(spawn_loading_screen));

    app.configure::<IsLoadingBarFill>();
}

fn spawn_loading_screen(mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands.entity(screen_root.ui).with_child(loading());
}

fn loading() -> impl Bundle {
    (
        Name::new("Loading"),
        Node::COLUMN_CENTER.full_size(),
        children![loading_text(), loading_bar()],
    )
}

fn loading_text() -> impl Bundle {
    (
        Name::new("LoadingText"),
        RichText::from_sections(parse_rich("[t]Loading...")),
        DynamicFontSize::new(Vw(5.0)).with_step(8.0),
        ThemeColorForText(vec![ThemeColor::BodyText]),
        Node {
            margin: UiRect::all(Percent(1.0)),
            ..default()
        },
    )
}

fn loading_bar() -> impl Bundle {
    (
        Name::new("LoadingBar"),
        Node {
            width: Percent(60.0),
            height: Percent(8.0),
            margin: UiRect::all(VMin(2.0)),
            padding: UiRect::all(VMin(1.0)),
            border: UiRect::all(VMin(1.0)),
            ..default()
        },
        ThemeColor::BodyText.set::<BorderColor>(),
        children![loading_bar_fill()],
    )
}

fn loading_bar_fill() -> impl Bundle {
    (
        Name::new("LoadingBarFill"),
        Node::DEFAULT.full_height(),
        ThemeColor::Primary.set::<BackgroundColor>(),
        IsLoadingBarFill,
    )
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

    // Continue to the next screen when ready.
    if done == total {
        commands.spawn(fade_out(Screen::Gameplay));
    }

    // Update loading bar fill.
    for mut node in &mut fill_query {
        node.width = Percent(100.0 * done as f32 / total as f32);
    }

    info!("[Frame {}] Loading: {done} / {total}", frame.0);
}
