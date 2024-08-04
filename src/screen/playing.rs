use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;
use leafwing_input_manager::prelude::*;
use pyri_state::prelude::*;

use crate::core::UpdateSet;
use crate::menu::Menu;
use crate::screen::Screen;
use crate::util::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(StateFlush, Screen::Playing.on_enter(spawn_playing_screen));

    app.configure::<(PlayingAssets, PlayingAction)>();
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
            Update,
            Screen::Playing.on_update(
                Menu::Pause.toggle().in_set(UpdateSet::RecordInput).run_if(
                    action_just_pressed(Self::TogglePause)
                        .and_then(Menu::is_disabled.or_else(Menu::Pause.will_update())),
                ),
            ),
        );
    }
}
