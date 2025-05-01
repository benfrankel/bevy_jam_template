use bevy::diagnostic::FrameCount;
use bevy::image::ImageLoaderSettings;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::FADE_IN_SECS;
use crate::screen::fade::fade_out;
use crate::screen::title::TitleScreenAssets;
use crate::screen::wait_in_screen;
use crate::theme::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_loading_state(
        LoadingState::new(Screen::Splash.bevy()).load_collection::<TitleScreenAssets>(),
    );
    app.add_systems(StateFlush, Screen::Splash.on_enter(spawn_splash_screen));

    app.add_systems(
        Update,
        Screen::Splash.on_update((
            wait_in_screen(FADE_IN_SECS + SPLASH_SCREEN_MIN_SECS),
            // TODO: System ordering so this runs after all the track progress systems.
            update_splash,
        )),
    );
}

const SPLASH_SCREEN_MIN_SECS: f32 = 0.8;

fn spawn_splash_screen(
    mut commands: Commands,
    screen_root: Res<ScreenRoot>,
    asset_server: Res<AssetServer>,
) {
    commands
        .entity(screen_root.ui)
        .with_child(splash(&asset_server));
}

fn splash(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Splash"),
        Node::COLUMN_MID.full_size(),
        children![splash_image(asset_server)],
    )
}

fn splash_image(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("SplashImage"),
        ImageNode::new(asset_server.load_with_settings(
            // TODO: Use `embedded_asset!` when https://github.com/bevyengine/bevy/issues/14246 is fixed.
            "image/splash.png",
            |settings: &mut ImageLoaderSettings| {
                settings.sampler = ImageSampler::linear();
            },
        )),
        Node {
            margin: UiRect::all(Auto),
            width: Percent(70.0),
            ..default()
        },
        ThemeColor::BodyText.set::<ImageNode>(),
    )
}

fn update_splash(
    mut commands: Commands,
    progress: Res<ProgressTracker<BevyState<Screen>>>,
    frame: Res<FrameCount>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.get_global_combined_progress();
    if *last_done == done {
        return;
    }
    *last_done = done;

    // Continue to the next screen when ready.
    if done == total {
        commands.spawn(fade_out(Screen::Title));
    }

    info!("[Frame {}] Booting: {done} / {total}", frame.0);
}
