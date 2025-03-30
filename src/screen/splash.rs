use bevy::asset::embedded_asset;
use bevy::core::FrameCount;
use bevy::image::ImageLoaderSettings;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::screen::fade::FADE_IN_SECS;
use crate::screen::fade::FadeOut;
use crate::screen::title::TitleScreenAssets;
use crate::screen::wait_in_screen;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    embedded_asset!(app, "splash/splash.png");

    app.add_loading_state(
        LoadingState::new(Screen::Splash.bevy()).load_collection::<TitleScreenAssets>(),
    );
    app.add_systems(StateFlush, Screen::Splash.on_enter(splash.spawn()));

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

fn splash(In(id): In<Entity>, mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands
        .entity(id)
        .insert(Node::COLUMN_MID.full_size().named("Splash"))
        .set_parent(screen_root.ui)
        .with_children(|parent| {
            parent.spawn_fn(splash_image);
        });
}

fn splash_image(In(id): In<Entity>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.entity(id).insert((
        Name::new("SplashImage"),
        ImageNode::new(asset_server.load_with_settings(
            "embedded://bevy_jam_template/screen/splash/splash.png",
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
    ));
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

    // Continue to next screen when ready
    if done == total {
        commands.spawn_with(FadeOut::to(Screen::Title));
    }

    info!("[Frame {}] Booting: {done} / {total}", frame.0);
}
