use crate::core::audio::AudioSettings;
use crate::core::audio::music_audio;
use crate::menu::Menu;
use crate::prelude::*;
use crate::screen::Screen;
use crate::screen::ScreenRoot;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Gameplay.on_enter(spawn_gameplay_screen));

    app.configure::<(GameplayAssets, GameplayAction)>();
}

fn spawn_gameplay_screen(
    mut commands: Commands,
    screen_root: Res<ScreenRoot>,
    audio_settings: Res<AudioSettings>,
    assets: Res<GameplayAssets>,
) {
    commands
        .entity(screen_root.ui)
        .with_child(widget::column_center(children![widget::label(
            "Gameplay goes here. Press P to pause!",
        )]));
    commands.spawn((
        music_audio(&audio_settings, assets.music.clone()),
        DespawnOnExitState::<Screen>::Recursive,
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
    TogglePause,
}

impl Configure for GameplayAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::TogglePause, GamepadButton::Start)
                .with(Self::TogglePause, KeyCode::Escape)
                .with(Self::TogglePause, KeyCode::KeyP),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            Update,
            Screen::Gameplay.on_update(
                Menu::Pause
                    .toggle()
                    .in_set(UpdateSystems::RecordInput)
                    .run_if(
                        action_just_pressed(Self::TogglePause)
                            .and(Menu::is_disabled.or(Menu::Pause.will_update())),
                    ),
            ),
        );
    }
}
