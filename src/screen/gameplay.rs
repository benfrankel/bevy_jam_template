use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRootUi;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        StateFlush,
        (
            Screen::Gameplay.on_enter(spawn_gameplay_screen),
            Menu::ANY.on_enter(
                spawn_menu_overlay.run_if(
                    Screen::Gameplay
                        .will_enter()
                        .and(Screen::is_triggered.or(Menu::is_disabled)),
                ),
            ),
        ),
    );

    app.configure::<(GameplayAssets, GameplayAction)>();
}

fn spawn_gameplay_screen(
    mut commands: Commands,
    screen_root_ui: Single<Entity, With<ScreenRootUi>>,
    audio_settings: Res<AudioSettings>,
    assets: Res<GameplayAssets>,
) {
    commands
        .entity(*screen_root_ui)
        .with_child(widget::center(children![widget::label(
            "Gameplay goes here. Press P to pause!",
        )]));
    commands.spawn((
        music_audio(&audio_settings, assets.music.clone()),
        DespawnOnExitState::<Screen>::Recursive,
    ));
}

fn spawn_menu_overlay(mut commands: Commands) {
    commands.spawn((
        widget::blocking_overlay(1),
        ThemeColor::Overlay.set::<BackgroundColor>(),
        DespawnOnExitState::<Screen>::Recursive,
        DespawnOnDisableState::<Menu>::Recursive,
    ));
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct GameplayAssets {
    #[asset(path = "audio/music/545458__bertsz__bit-forest-evil-theme-music.ogg")]
    music: Handle<AudioSource>,
}

impl Configure for GameplayAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

#[derive(Actionlike, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum GameplayAction {
    Pause,
    CloseMenu,
}

impl Configure for GameplayAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::Pause, GamepadButton::Start)
                .with(Self::Pause, KeyCode::Escape)
                .with(Self::Pause, KeyCode::KeyP)
                .with(Self::CloseMenu, KeyCode::KeyP),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Screen::Gameplay.on_update((
                Menu::Pause
                    .enter()
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_disabled.and(action_just_pressed(Self::Pause))),
                Menu::clear
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(Menu::is_enabled.and(action_just_pressed(Self::CloseMenu))),
            )),
        );
    }
}
