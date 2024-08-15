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
    app.add_systems(StateFlush, Screen::Playing.on_enter(playing.spawn()));

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

fn playing(In(id): In<Entity>, mut commands: Commands) {
    // TODO: Spawn HUD.
    commands.entity(id);
}

#[derive(Actionlike, Copy, Clone, Eq, PartialEq, Hash, Reflect, Debug)]
pub enum PlayingAction {
    TogglePause,
}

impl Configure for PlayingAction {
    fn configure(app: &mut App) {
        app.init_resource::<ActionState<Self>>();
        app.insert_resource(
            InputMap::default()
                .with(Self::TogglePause, GamepadButtonType::Start)
                .with(Self::TogglePause, KeyCode::Escape)
                .with(Self::TogglePause, KeyCode::KeyP),
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
