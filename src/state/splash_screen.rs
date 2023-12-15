use std::f32::consts::PI;

use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use bevy::render::texture::ImageSampler;
use bevy::render::texture::ImageType;
use bevy::ui::Val::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::state::title_screen::TitleScreenAssets;
use crate::state::AppState::*;
use crate::theme::PaletteColor;
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

        app.register_type::<SplashScreenStartTime>()
            .register_type::<SplashImageFadeInOut>()
            .add_systems(
                Update,
                // TODO: This has to run after apply_palette_colors
                update_splash_screen
                    .track_progress()
                    .run_if(in_state(SplashScreen)),
            );

        app.add_loading_state(LoadingState::new(SplashScreen))
            .add_collection_to_loading_state::<_, TitleScreenAssets>(SplashScreen)
            .add_plugins(ProgressPlugin::new(SplashScreen).continue_to(TitleScreen))
            .add_systems(OnEnter(SplashScreen), enter_splash_screen)
            .add_systems(OnExit(SplashScreen), exit_splash_screen);
    }
}

const SPLASH_SCREEN_MIN_SECS: f64 = 2.0;
const SPLASH_SCREEN_IMAGE_HANDLE: Handle<Image> =
    Handle::weak_from_u128(145948501136218819748366695396142082634);

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct SplashScreenStartTime(f64);

fn enter_splash_screen(mut commands: Commands, root: Res<AppRoot>, time: Res<Time>) {
    commands.insert_resource(SplashScreenStartTime(time.elapsed_seconds_f64()));

    let screen = spawn_splash_screen(&mut commands);
    commands.entity(screen).set_parent(root.ui);
}

fn exit_splash_screen(mut commands: Commands, root: Res<AppRoot>) {
    commands.remove_resource::<SplashScreenStartTime>();
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
            PaletteColor::Foreground,
            SplashImageFadeInOut,
        ))
        .set_parent(screen);

    screen
}

#[derive(Component, Reflect)]
struct SplashImageFadeInOut;

// TODO: Replace this with some Animation component
fn update_splash_screen(
    mut color_query: Query<&mut BackgroundColor, With<SplashImageFadeInOut>>,
    time: Res<Time>,
    start: Res<SplashScreenStartTime>,
) -> Progress {
    let elapsed = (time.elapsed_seconds_f64() - start.0) / SPLASH_SCREEN_MIN_SECS;

    for mut color in &mut color_query {
        let t = elapsed as f32;
        let amplitude = 1.5;
        let alpha = (amplitude * (PI * t).sin() - amplitude + 1.0)
            .max(0.0)
            .powf(1.2);
        color.0.set_a(alpha);
    }

    (elapsed >= 1.0).into()
}
