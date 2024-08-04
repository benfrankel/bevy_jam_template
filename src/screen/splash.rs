use bevy::asset::embedded_asset;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::render::texture::ImageLoaderSettings;
use bevy::render::texture::ImageSampler;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;
use pyri_state::prelude::*;

use crate::screen::fade::FadeOut;
use crate::screen::fade::FADE_IN_SECS;
use crate::screen::title::TitleScreenAssets;
use crate::screen::wait_in_screen;
use crate::screen::Screen;
use crate::screen::ScreenRoot;
use crate::theme::prelude::*;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    embedded_asset!(app, "splash/splash.png");

    app.add_loading_state(
        LoadingState::new(Screen::Splash.bevy()).load_collection::<TitleScreenAssets>(),
    );
    app.add_plugins(ProgressPlugin::new(Screen::Splash.bevy()));
    app.add_systems(StateFlush, Screen::Splash.on_enter(splash.spawn()));

    app.add_systems(
        Update,
        Screen::Splash.on_update((
            wait_in_screen(FADE_IN_SECS + SPLASH_SCREEN_MIN_SECS),
            update_splash.after(TrackedProgressSet),
        )),
    );
}

const SPLASH_SCREEN_MIN_SECS: f32 = 0.8;

fn splash(In(id): In<Entity>, mut commands: Commands, screen_root: Res<ScreenRoot>) {
    commands
        .entity(id)
        .insert(Style::COLUMN_MID.full_size().node("Splash"))
        .set_parent(screen_root.ui)
        .with_children(|children| {
            children.spawn_fn(splash_image);
        });
}

fn splash_image(In(id): In<Entity>, mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.entity(id).insert((
        Name::new("SplashImage"),
        ImageBundle {
            style: Style {
                margin: UiRect::all(Auto),
                width: Percent(70.0),
                ..default()
            },
            image: UiImage::new(asset_server.load_with_settings(
                "embedded://bevy_jam_template/screen/splash/splash.png",
                |settings: &mut ImageLoaderSettings| {
                    settings.sampler = ImageSampler::linear();
                },
            )),
            ..default()
        },
        ThemeColor::BodyText.set::<UiImage>(),
    ));
}

fn update_splash(
    mut commands: Commands,
    progress: Res<ProgressCounter>,
    frame: Res<FrameCount>,
    mut last_done: Local<u32>,
) {
    let Progress { done, total } = progress.progress();
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
