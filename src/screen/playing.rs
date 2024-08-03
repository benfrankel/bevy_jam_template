pub mod pause_menu;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;
use pyri_state::schedule::ResolveStateSet;

use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Playing.on_enter(spawn_playing_screen));

    app.configure::<(PlayingAssets, PlayingAction, PlayingMenu)>();
}

#[derive(AssetCollection, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayingAssets {}

impl Configure for PlayingAssets {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.init_collection::<Self>();
    }
}

fn spawn_playing_screen(mut _commands: Commands) {
    // TODO
}

#[derive(Actionlike, Reflect, Clone, Hash, PartialEq, Eq)]
pub enum PlayingAction {
    TogglePause,
}

impl Configure for PlayingAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .insert(Self::TogglePause, GamepadButtonType::Start)
                .insert(Self::TogglePause, KeyCode::Escape)
                .insert(Self::TogglePause, KeyCode::KeyP)
                .build(),
        );
        app.add_plugins(InputManagerPlugin::<Self>::default());
        app.add_systems(
            StateFlush,
            PlayingMenu::Pause
                .toggle()
                .in_set(ResolveStateSet::<PlayingMenu>::Compute)
                .run_if(
                    PlayingMenu::is_disabled
                        .or_else(PlayingMenu::Pause.will_exit())
                        .and_then(Screen::Playing.will_enter())
                        .and_then(action_just_pressed(Self::TogglePause)),
                ),
        );
    }
}

#[derive(State, Copy, Clone, Eq, PartialEq, Debug, Reflect)]
#[state(after(Screen), react, log_flush)]
#[reflect(Resource)]
enum PlayingMenu {
    Pause,
    Victory,
    Defeat,
}

impl Configure for PlayingMenu {
    fn configure(app: &mut App) {
        app.register_type::<Self>();
        app.add_state::<Self>();
        app.add_systems(StateFlush, Screen::Playing.on_exit(Self::disable));
        app.add_plugins(pause_menu::plugin);
    }
}
