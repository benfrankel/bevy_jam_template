use bevy::asset::load_internal_binary_asset;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::texture::ImageSampler;
use bevy::render::texture::ImageType;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::core::theme::ThemeBackgroundColor;
use crate::core::theme::ThemeColor;
use crate::screen::fade_in;
use crate::screen::fade_out;
use crate::screen::title::TitleScreenAssets;
use crate::screen::Screen;
use crate::screen::FADE_IN_SECS;
use crate::util::ui::UiRoot;
use crate::util::wait;

pub struct SplashScreenplugin;

impl Plugin for SplashScreenplugin {
    fn build(&self, app: &mut App) {
        load_internal_binary_asset!(
            app,
            SPLASH_SCREEN_IMAGE_HANDLE,
            "../../assets/image/ui/splash.png",
            |bytes, _path: String| {
                Image::from_buffer(
                    bytes,
                    ImageType::Extension("png"),
                    default(),
                    true,
                    ImageSampler::linear(),
                    RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
                )
                .unwrap()
            }
        );

        app.add_loading_state(
            LoadingState::new(Screen::Splash).load_collection::<TitleScreenAssets>(),
        )
        .add_plugins(ProgressPlugin::new(Screen::Splash))
        .add_systems(OnEnter(Screen::Splash), enter_splash_screen)
        .add_systems(OnExit(Screen::Splash), exit_splash_screen);

        app.add_systems(
            Update,
            (
                wait(FADE_IN_SECS + SPLASH_SCREEN_MIN_SECS),
                update_splash.after(TrackedProgressSet),
            )
                .run_if(in_state(Screen::Splash)),
        );
    }
}

const SPLASH_SCREEN_MIN_SECS: f32 = 1.5;
const SPLASH_SCREEN_IMAGE_HANDLE: Handle<Image> =
    Handle::weak_from_u128(145948501136218819748366695396142082634);

fn enter_splash_screen(mut commands: Commands, ui_root: Res<UiRoot>) {
    fade_in(&mut commands);

    let screen = spawn_splash_screen(&mut commands);
    commands.entity(screen).set_parent(ui_root.body);
}

fn exit_splash_screen(mut commands: Commands, ui_root: Res<UiRoot>) {
    commands.entity(ui_root.body).despawn_descendants();
}

fn spawn_splash_screen(commands: &mut Commands) -> Entity {
    let screen = commands
        .spawn((
            Name::new("SplashScreen"),
            NodeBundle {
                style: Style {
                    width: Percent(100.0),
                    height: Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Name::new("SplashImage"),
            ImageBundle {
                style: Style {
                    margin: UiRect::all(Auto),
                    width: Percent(70.0),
                    ..default()
                },
                image: UiImage::new(SPLASH_SCREEN_IMAGE_HANDLE),
                ..default()
            },
            ThemeBackgroundColor(ThemeColor::BodyText),
        ))
        .set_parent(screen);

    screen
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
        fade_out(&mut commands, Screen::Title);
    }

    info!("[Frame {}] Booting: {done} / {total}", frame.0);
}
