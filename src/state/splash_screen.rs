use bevy::asset::load_internal_binary_asset;
use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use bevy::render::texture::ImageType;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::state::title_screen::TitleScreenAssets;
use crate::state::AppState::*;
use crate::state::FADE_IN_SECS;
use crate::state::FADE_OUT_SECS;
use crate::theme::ThemeBackgroundColor;
use crate::theme::ThemeColor;
use crate::ui::fade_in;
use crate::ui::fade_out;
use crate::util::wait;
use crate::AppRoot;

pub struct SplashScreenStatePlugin;

impl Plugin for SplashScreenStatePlugin {
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
                )
                .unwrap()
            }
        );

        app.add_loading_state(LoadingState::new(SplashScreen))
            .add_collection_to_loading_state::<_, TitleScreenAssets>(SplashScreen)
            .add_plugins(ProgressPlugin::new(SplashScreen))
            .add_systems(OnEnter(SplashScreen), enter_splash_screen)
            .add_systems(OnExit(SplashScreen), exit_splash_screen);

        app.add_systems(
            Update,
            (
                wait(FADE_IN_SECS + SPLASH_SCREEN_MIN_SECS).track_progress(),
                update_splash.after(TrackedProgressSet),
            )
                .run_if(in_state(SplashScreen)),
        );
    }
}

const SPLASH_SCREEN_MIN_SECS: f32 = 1.5;
const SPLASH_SCREEN_IMAGE_HANDLE: Handle<Image> =
    Handle::weak_from_u128(145948501136218819748366695396142082634);

fn enter_splash_screen(mut commands: Commands, root: Res<AppRoot>) {
    fade_in(&mut commands, FADE_IN_SECS);

    let screen = spawn_splash_screen(&mut commands);
    commands.entity(screen).set_parent(root.ui);
}

fn exit_splash_screen(mut commands: Commands, root: Res<AppRoot>) {
    commands.entity(root.ui).despawn_descendants();
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

    // Continue to next state when ready
    if done == total {
        fade_out(&mut commands, FADE_OUT_SECS, TitleScreen);
    }

    info!("[Frame {}] Booting: {done} / {total}", frame.0);
}
